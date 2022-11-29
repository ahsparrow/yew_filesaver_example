use gloo_file::{Blob, ObjectUrl};
use gloo_utils::document;
use wasm_bindgen::JsCast;
use yew::{function_component, html, Callback, Html, Renderer};

#[function_component(App)]
fn app() -> Html {
    let onsave = {
        Callback::from(move |_| {
            let download_anchor = document()
                .get_element_by_id("download")
                .unwrap()
                .dyn_into::<web_sys::HtmlAnchorElement>()
                .unwrap();

            let blob = Blob::new("Hello World");
            let object_url = ObjectUrl::from(blob);

            download_anchor.set_href(&object_url);
            download_anchor.click();
        })
    };

    html! {
        <>
            <button onclick={onsave}>{"Press me"}</button>
            <a id="download" hidden=true download="test.txt"></a>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<App>::new().render();
}
