use yew::prelude::*;

mod components;
mod services;
use components::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Nav/>
        <AppList/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
