use yew::prelude::*;

use crate::data::portfolio::SKILLS;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" aria-labelledby="about-title">
            <p class="eyebrow">{"About"}</p>
            <h2 id="about-title">{"Building products that feel simple, even when the systems behind them are not."}</h2>
            <p>{"I’m a full-stack developer who enjoys working where product thinking and engineering meet. My work spans native iOS apps, APIs, background processing, and cloud infrastructure."}</p>
            <p>{"I care about clear interfaces, resilient systems, and shipping useful things that can keep improving after launch."}</p>
            <div class="skill-list" aria-label="Tools I use">
                {for SKILLS.iter().map(|skill| html! { <span>{skill}</span> })}
            </div>
        </section>
    }
}
