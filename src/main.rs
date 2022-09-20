// use std::ops::Deref;

// use gloo_timers::callback::Interval;
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

// #[function_component(App)]
// fn app() -> Html {
//     let state = use_state(|| 0);
//     {
//         let state = state.clone();
//         use_effect_with_deps(
//             move |_| {
//                 Interval::new(1000, move || {
//                     state.set(*state + 1);
//                 })
//                 .forget();
//                 || ()
//             },
//             (),
//         );
//     }

//     html! {
//         <p>{{*state}}</p>
//     }
// }

fn main() {
    yew::start_app::<App>();
}
