use leptos::prelude::{RwSignal, Set};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    fn tauri_invoke(cmd: &str, args: JsValue) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen)]
    async fn listen_event(event: &str, handler: &Closure<dyn FnMut(JsValue)>);
}

#[derive(Serialize)]
struct AddArgs {
    left: i32,
    right: i32,
}

pub async fn invoke_add(left: i32, right: i32) -> Result<i32, String> {
    let args = AddArgs { left, right };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;

    let p = tauri_invoke("add_numbers", args_js);
    let v = JsFuture::from(p).await.map_err(|e| format!("{e:?}"))?;

    serde_wasm_bindgen::from_value(v).map_err(|e| e.to_string())
}

pub async fn emit_message(emit_message: RwSignal<String>) {
    let emit_message = emit_message.clone();

    let cb = Closure::wrap(Box::new(move |event: JsValue| {
        // event.payload.width / height を読む
        let payload = js_sys::Reflect::get(&event, &JsValue::from_str("payload")).unwrap();
        let w = js_sys::Reflect::get(&payload, &JsValue::from_str("width")).unwrap();
        let h = js_sys::Reflect::get(&payload, &JsValue::from_str("height")).unwrap();

        let w = w.as_f64().unwrap() as u32;
        let h = h.as_f64().unwrap() as u32;

        emit_message.set(format!("width={w}, height={h}"));
    }) as Box<dyn FnMut(_)>);

    listen_event("window-resized", &cb).await;
    cb.forget(); // 最小構成なので忘れる
}