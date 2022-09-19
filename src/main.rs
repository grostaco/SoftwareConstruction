use std::env;

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Nav/>
        <AppList/>
        </>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <>
        <nav class="navbar dflex-justify-center">
            <div class="bold font-lg">{{"Software Construction"}}</div>
            <div class="dflex dflex-row dflex-gap-md">
                <div>{{"Home"}}</div>
                <div>{{"GitHub"}}</div>
                <div>{{"About Me"}}</div>
            </div>
        </nav>
        <div class="divider"></div>
        </>
    }
}

#[function_component(AppList)]
fn app_list() -> Html {
    let assignments = use_async_with_options(
        async move {
            let mut assignments = Vec::new();
            let mut i = 1;
            loop {
                let url = format!(
                    "https://grostaco.github.io/SoftwareConstruction/assignment{}/index.html",
                    i
                );
                let res = reqwest::get(&url).await.unwrap();
                if res.status().as_u16() == 200 {
                    let about = match reqwest::get(format!(
                        "https://grostaco.github.io/SoftwareConstruction/assignment{i}/about.txt"
                    ))
                    .await
                    .map(|res| async {
                        res.text().await.unwrap_or(
                            "Cannot load the information about this assignment".to_string(),
                        )
                    }) {
                        Ok(text) => text.await,
                        Err(_) => "Cannot load about.txt".to_string(),
                    };

                    assignments.push(html! {
                        <div class="dflex dflex-col dflex-gap-md" style="margin-top: 2rem;">
                            <div class="dflex dflex-row dflex-gap-md">
                                <img src="placeholder.png" width="128px" height="128px"/>
                                <div class="dflex dflex-col dflex-justify-between">
                                    <div class="dflex dflex-col dflex-gap-sm">
                                        <div class="bold">{{"Assignment 1"}}</div>
                                        <div>{about}</div>
                                    </div>
                                    <div class="dflex dflex-row dflex-gap-sm">
                                        <a href={{url}}>{{"Live Preview"}}</a>
                                        <a href={{format!("https://github.com/grostaco/SoftwareConstruction/tree/main/vue/assignment{i}")}}>{{"View Code"}}</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    });
                } else {
                    break;
                }
                i += 1;
            }
            Ok::<_, ()>(assignments)
        },
        UseAsyncOptions::enable_auto(),
    );

    html! {
        <main class="main">
            <h class="bold font-md" style="color: white">{{ "Assignments" }}</h>
            if let Some(assignments) = assignments.data.clone() {
                { for assignments }
            } else {
                <div>
                    {{"Loading..."}}
                </div>
            }
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
