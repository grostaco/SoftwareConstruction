use yew::{function_component, html};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::assignment::get_assignments;

#[function_component(AppList)]
pub fn app_list() -> Html {
    let assignments = use_async_with_options(
        async move {
            let assignments = get_assignments().await.into_iter().map(|assignment| html! {
                        <div class="dflex dflex-col dflex-gap-md" style="margin-top: 2rem;">
                            <div class="dflex dflex-row dflex-gap-md">
                                <img src="placeholder.png" width="128px" height="128px"/>
                                <div class="dflex dflex-col dflex-justify-between">
                                    <div class="dflex dflex-col dflex-gap-sm">
                                        <div class="bold">{{format!("Assignment {}", assignment.number)}}</div>
                                        <div>{assignment.about.unwrap()}</div>
                                    </div>
                                    <div class="dflex dflex-row dflex-gap-sm">
                                        <a class="dflex dflex-justify-center dflex-row" style="gap: 10px;" href={{format!("https://grostaco.github.io/SoftwareConstruction/assignment{}/index.html", assignment.number)}}>
                                            {{"Live Preview"}}
                                            <svg version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" viewBox="0 0 60 60" height="24" width="24" style="enable-background:new 0 0 60 60; fill: #9eaab7;">
                                                <path d="M45.563,29.174l-22-15c-0.307-0.208-0.703-0.231-1.031-0.058C22.205,14.289,22,14.629,22,15v30
                                                    c0,0.371,0.205,0.711,0.533,0.884C22.679,45.962,22.84,46,23,46c0.197,0,0.394-0.059,0.563-0.174l22-15
                                                    C45.836,30.64,46,30.331,46,30S45.836,29.36,45.563,29.174z M24,43.107V16.893L43.225,30L24,43.107z"/>
                                                <path d="M30,0C13.458,0,0,13.458,0,30s13.458,30,30,30s30-13.458,30-30S46.542,0,30,0z M30,58C14.561,58,2,45.439,2,30
                                                    S14.561,2,30,2s28,12.561,28,28S45.439,58,30,58z"/>
                                            </svg>
                                        </a>
                                        <a class="dflex dflex-justify-center dflex-row" style="gap: 10px;" href={{format!("https://github.com/grostaco/SoftwareConstruction/tree/main/vue/assignment{}", assignment.number)}}>
                                            {{"View Code"}}
                                            <svg style="fill: #9eaab7" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
                                        </a>
                                        
                                    </div>
                                </div>
                            </div>
                        </div>
                    }).collect::<Vec<_>>();
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
