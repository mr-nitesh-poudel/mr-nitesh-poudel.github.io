use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::icons;
use crate::{data::portfolio::CONTACT_EMAIL, services::clipboard};

#[derive(Clone, Copy, PartialEq)]
enum CopyStatus {
    Ready,
    Copied,
    Failed,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let copy_status = use_state(|| CopyStatus::Ready);
    let onclick = {
        let copy_status = copy_status.clone();
        Callback::from(move |_| {
            let copy_status = copy_status.clone();
            spawn_local(async move {
                let next_status = if clipboard::copy(CONTACT_EMAIL).await {
                    CopyStatus::Copied
                } else {
                    CopyStatus::Failed
                };
                copy_status.set(next_status);
                TimeoutFuture::new(1_800).await;
                copy_status.set(CopyStatus::Ready);
            });
        })
    };
    let (button_label, accessible_label) = match *copy_status {
        CopyStatus::Ready => ("Copy", "Copy email address"),
        CopyStatus::Copied => ("Copied!", "Email address copied"),
        CopyStatus::Failed => ("Try again", "Could not copy email address"),
    };

    html! {
        <section id="contact" aria-label="Contact">
            <p class="eyebrow">{"Contact"}</p>
            <p>{"Have a product or technical problem to solve? My inbox is open."}</p>
            <div class="email-actions">
                <a class="email" href={format!("mailto:{CONTACT_EMAIL}")}>{CONTACT_EMAIL}{icons::external_link()}</a>
                <button class="copy-email" type="button" aria-label={accessible_label} {onclick}>{button_label}</button>
            </div>
            <footer>{"© 2026 Nitesh Poudel · Built with Yew"}</footer>
        </section>
    }
}
