use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="navbar is-size-3">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">{"TBR"}</a>
            </div>
            <div class="navbar-menu">
                <div class="navbar-end">
                    <a class="navbar-item" href="/#about">{"About"}</a>
                    <a class="navbar-item" href="/projects">{"Projects"}</a>
                    <a class="navbar-item" href="/contact">{"Contact"}</a>
                </div>
            </div>
        </nav>
    }
}
