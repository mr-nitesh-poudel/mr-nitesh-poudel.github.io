use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub async fn copy(text: &str) -> bool {
    let Some(window) = window() else {
        return false;
    };

    JsFuture::from(window.navigator().clipboard().write_text(text))
        .await
        .is_ok()
}
