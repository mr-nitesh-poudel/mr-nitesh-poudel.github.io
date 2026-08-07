use yew::prelude::*;

use crate::data::portfolio::PROJECTS;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <section id="work" aria-labelledby="work-title">
            <p class="eyebrow">{"Selected work"}</p>
            <h2 id="work-title">{"Projects I’ve enjoyed building."}</h2>
            <div class="projects">
                {for PROJECTS.iter().enumerate().map(|(index, project)| html! {
                    <article class="project">
                        <span class="number">{format!("0{}", index + 1)}</span>
                        <div>
                            <p class="project-kind">{project.kind}</p>
                            <h3>{project.title}</h3>
                            <p>{project.description}</p>
                            <ul aria-label={format!("{} technologies", project.title)}>
                                {for project.stack.iter().map(|item| html! { <li>{item}</li> })}
                            </ul>
                        </div>
                    </article>
                })}
            </div>
        </section>
    }
}
