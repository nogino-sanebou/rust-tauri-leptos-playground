use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::invoke_emit::{emit_message, invoke_add};

#[component]
pub fn InvokeEmit() -> impl IntoView {
    let left = RwSignal::<String>::new(String::new());
    let right = RwSignal::<String>::new(String::new());

    let result = RwSignal::<Option<i32>>::new(None);
    let error = RwSignal::<Option<String>>::new(None);

    let click = move |_| {
        let left = left.get().parse::<i32>();
        let right = right.get().parse::<i32>();

        spawn_local(async move {
            match invoke_add(left.unwrap(), right.unwrap()).await {
                Ok(sum) => result.set(Some(sum)),
                Err(e) => error.set(Some(e)),
            }
        });
    };

    let out_result = move || {
        match (result.get(), error.get()) {
            (_, Some(e)) => view! { <span>"エラー："{e}</span> }.into_any(),
            (Some(v), None) => view! { <span>"結果："{v}</span> }.into_any(),
            (None, None) => view! { <span>"　　　　　"</span>}.into_any(),
        }
    };

    let emit_msg = RwSignal::<String>::new(String::from("ウィンドウサイズ"));
    spawn_local(async move {
        let _ = emit_message(emit_msg).await;
    });

    view! {
        <h1>"Invoke and Emit"</h1>
        <h2>"invoke"</h2>
        <div style="display: flex; flex-direction: column; gap: 5px;">
            <div style="display: flex; gap: 3px;">
                <input
                    type="text"
                    prop:value=move || left.get()
                    on:input=move |ev| left.set(event_target_value(&ev))
                />
                <span>"+"</span>
                <input
                    type="text"
                    prop:value=move || right.get()
                    on:input=move |ev| right.set(event_target_value(&ev))
                />
                <button on:click=click>"invoke"</button>
            </div>
            <div style="border: 3px solid black">{out_result}</div>
        </div>
        <h2>"emit"</h2>
        <div style="background-color: yellow; width: 200px; height: 25px">{emit_msg}</div>
    }
}