use yew::prelude::*;
use yew_router::prelude::*;

use log::Level;

mod routes;
use crate::routes::{switch, Route};

mod components;
mod pages;
mod projects;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).expect("Failed to init logger");
    yew::Renderer::<App>::new().render();
}
