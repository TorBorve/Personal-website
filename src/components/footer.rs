use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="content has-text-centered">
                { "© 2025 Tor Børve Rasmussen" }
            </div>
        </footer>
    }
}