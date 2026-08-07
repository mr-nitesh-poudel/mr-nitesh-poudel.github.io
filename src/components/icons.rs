use yew::prelude::*;
use yew_icons::{Icon, IconData};

const ICON_SIZE: &str = "1rem";

fn render(data: IconData, class: &'static str) -> Html {
    html! {
        <Icon
            {data}
            class={class}
            width={ICON_SIZE.to_owned()}
            height={ICON_SIZE.to_owned()}
        />
    }
}

pub fn github() -> Html {
    render(IconData::SIMPLE_ICONS_GITHUB, "icon")
}

pub fn whatsapp() -> Html {
    render(IconData::SIMPLE_ICONS_WHATSAPP, "icon")
}

pub fn email() -> Html {
    render(IconData::LUCIDE_MAIL, "icon")
}

pub fn sun() -> Html {
    render(
        IconData::LUCIDE_SUN,
        "theme-toggle__icon theme-toggle__icon--sun",
    )
}

pub fn moon() -> Html {
    render(
        IconData::LUCIDE_MOON,
        "theme-toggle__icon theme-toggle__icon--moon",
    )
}

pub fn external_link() -> Html {
    render(IconData::LUCIDE_ARROW_UP_RIGHT, "external-link-icon")
}
