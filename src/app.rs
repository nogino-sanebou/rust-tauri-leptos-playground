use leptos::*;
use leptos::prelude::*;
use crate::components::invoke_emit::InvokeEmit;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <InvokeEmit />
        </main>
    }
}
