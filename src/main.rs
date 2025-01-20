use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{HomePage, ContactPage};


mod components;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/contact")]
    Contact,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <HomePage/>},
        Route::Contact => html!{ <ContactPage/>}
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }

}



fn main() {
    yew::Renderer::<App>::new().render();
}
