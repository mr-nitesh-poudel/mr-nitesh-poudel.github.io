use yew::prelude::*;

use super::icons;
use crate::utils::theme::{self, Theme};

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let is_dark = use_state(theme::is_dark);
    let onclick = {
        let is_dark = is_dark.clone();
        Callback::from(move |_| {
            let next_theme = if *is_dark { Theme::Light } else { Theme::Dark };
            theme::set(next_theme);
            is_dark.set(next_theme == Theme::Dark);
        })
    };
    let label = if *is_dark {
        "Switch to light theme"
    } else {
        "Switch to dark theme"
    };

    html! {
        <button class="theme-toggle" type="button" aria-label={label} aria-pressed={is_dark.to_string()} {onclick}>
            {icons::sun()}
            {icons::moon()}
        </button>
    }
}
