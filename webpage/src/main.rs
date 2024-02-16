use leptos::{
    component, create_node_ref, create_resource, create_signal, html::Input, view, IntoView,
    NodeRef, ReadSignal, SignalGet, SignalSet, WriteSignal,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (file, set_file) = create_signal(Option::<String>::None);

    view! { <UploadButton file=file set_file=set_file/>
        <p>"Name is: " {move || file.get()}</p>
    }
}

#[component]
fn UploadButton(
    file: ReadSignal<Option<String>>,
    set_file: WriteSignal<Option<String>>,
) -> impl IntoView {
    let input_element: NodeRef<Input> = create_node_ref();
    let (future_signal, set_future_signal) = create_signal(None);
    let async_data = create_resource(
        || future_signal.get(),
        |promise| async move {
            if let Some(p) = promise {
                let resolved: Result<JsValue, JsValue> = p.await;
                set_file.set(resolved.ok().and_then(|x| x.as_string()));
            }
        },
    );
    let on_change = move |ev| {
        let value = input_element
            .get()
            .expect("<input> to exist")
            .files()
            .and_then(|fs| fs.get(0))
            .map(|f| JsFuture::from(f.text()));
        set_future_signal.set(value);
    };
    view! {
        <input
            type="file"
            on:change=on_change/>

    }
}
