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
                                        <a href={{format!("https://grostaco.github.io/SoftwareConstruction/assignment{}/index.html", assignment.number)}}>{{"Live Preview"}}</a>
                                        <a href={{format!("https://github.com/grostaco/SoftwareConstruction/tree/main/vue/assignment{}", assignment.number)}}>{{"View Code"}}</a>
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
