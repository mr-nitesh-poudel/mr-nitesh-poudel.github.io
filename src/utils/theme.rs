use web_sys::window;

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

pub fn is_dark() -> bool {
    let Some(window) = window() else { return true };
    let Some(root) = window
        .document()
        .and_then(|document| document.document_element())
    else {
        return true;
    };

    match root.get_attribute("data-theme").as_deref() {
        Some("dark") => true,
        Some("light") => false,
        _ => window
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .is_some_and(|query| query.matches()),
    }
}

pub fn set(theme: Theme) {
    let Some(window) = window() else { return };
    if let Some(root) = window
        .document()
        .and_then(|document| document.document_element())
    {
        let _ = root.set_attribute("data-theme", theme.as_str());
    }
    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item("theme", theme.as_str());
    }
}
