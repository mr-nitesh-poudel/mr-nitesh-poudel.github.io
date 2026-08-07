use yew::prelude::*;

use super::{ThemeToggle, icons};
use crate::data::portfolio::CONTACT_EMAIL;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <header class="intro" id="top">
            <div>
                <a class="name" href="#top">{"Nitesh Poudel"}</a>
                <p class="role">{"Software Engineer"}</p>
                <p class="summary">{"I build thoughtful mobile products, dependable backends, and the infrastructure that connects them."}</p>
                <nav aria-label="Primary navigation">
                    <span class="nav-label">{"Site index"}</span>
                    <div class="nav-grid">
                        <a href="#about">{"About"}</a>
                        <a href="#work">{"Work"}</a>
                        <a href="#contact">{"Contact"}</a>
                    </div>
                </nav>
            </div>
            <div class="intro-footer">
                <div class="socials" aria-label="Social links">
                    <a href="https://github.com/mr-nitesh-poudel" target="_blank" rel="noopener" aria-label="GitHub">{icons::github()}</a>
                    <a href="https://wa.me/" target="_blank" rel="noopener" aria-label="WhatsApp">{icons::whatsapp()}</a>
                    <a href={format!("mailto:{CONTACT_EMAIL}")} aria-label="Email">{icons::email()}</a>
                </div>
                <ThemeToggle />
            </div>
        </header>
    }
}
