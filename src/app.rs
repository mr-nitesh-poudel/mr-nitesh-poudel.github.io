use yew::prelude::*;

use crate::components::{About, Contact, Projects, Sidebar};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="page-shell">
            <Sidebar />
            <main>
                <About />
                <Projects />
                <Contact />
            </main>
        </div>
    }
}
