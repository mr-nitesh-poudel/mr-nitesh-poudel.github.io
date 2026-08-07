mod app;
mod components;
mod data;
mod services;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
