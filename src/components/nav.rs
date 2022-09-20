use yew::{function_component, html};

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <>
        <nav class="navbar dflex-justify-center">
            <div class="bold font-lg">{{"Software Construction"}}</div>
            <div class="dflex dflex-row dflex-gap-md">
                <a href="#">{{"Home"}}</a>
                <a href="https://github.com/grostaco">{{"GitHub"}}</a>
                <a href="https://https://grostaco.herokuapp.com/">{{"About Me"}}</a>
                <a href="https://github.com/grostaco/SoftwareConstruction">{{"This site's code"}}</a>
            </div>
        </nav>
        <div class="divider"></div>
        </>
    }
}