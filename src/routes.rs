use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    ContactPage, HomePage, NotFoundPage, ProjectDetailPage, ProjectsListPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/projects")]
    ProjectsList,
    #[at("/projects/:id")]
    ProjectDetail { id: usize },
    #[not_found]
    #[at("/not_found")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/>},
        Route::Contact => html! { <ContactPage/>},
        Route::ProjectsList => html! { <ProjectsListPage/>},
        Route::ProjectDetail { id } => html! { <ProjectDetailPage id={id}/>},
        Route::NotFound => html! { <NotFoundPage/>},
    }
}
