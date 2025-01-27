use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{HomePage, ContactPage, ProjectsListPage, ProjectDetailPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/contact")]
    Contact,
    #[at("/projects")]
    ProjectsList,
    #[at("/projects/:id")]
    ProjectDetail {id: usize},
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <HomePage/>},
        Route::Contact => html!{ <ContactPage/>},
        Route::ProjectsList => html!{ <ProjectsListPage/>},
        Route::ProjectDetail { id } => html!{ <ProjectDetailPage id={id}/>}
    }
}