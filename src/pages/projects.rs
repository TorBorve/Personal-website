use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    components::{Footer, NavBar},
    projects::{load_all_projects, Project},
    routes::Route,
};

#[derive(Properties, PartialEq, Clone)]
struct ProjectCardProps {
    project: Project,
    id: usize,
}

#[function_component(ProjectCard)]
fn project_card(props: &ProjectCardProps) -> Html {
    let ProjectCardProps { project, id } = props.clone();

    let formated_date = project.date.format("%B %d, %Y").to_string();

    html! {
        <Link<Route> to={Route::ProjectDetail {id}}>
        <div class="project-card">
            <div class="columns">
                <div class="column is-one-third">
                    <figure class="image is-1by1">
                        <img src={project.image_url.clone()} alt={format!("Image of {}", project.title)}/>
                    </figure>
                </div>
                <div class="column is-flex is-flex-direction-column is-justify-content-space-between">
                    <div>
                        <h1 class="title title-bar is-size-2">{&project.title}</h1>
                        <br/>
                        <h2 class="subtitle is-size-3">{&project.summary}</h2>
                    </div>
                    <div class="">
                        <h1 class="title is-size-5 has-text-right">{formated_date}</h1>
                    </div>
                </div>
            </div>
        </div>
        </Link<Route>>
    }
}

#[function_component(ProjectList)]
fn project_list() -> Html {
    let projects = load_all_projects();

    html! {
        <div class="container">
            <div class="columns is-multiline">
                {for projects.into_iter().enumerate().map(|(index, project)| {
                    let project_clone = project.clone();
                    html!{
                        <div class="column is-full">
                            <ProjectCard project={project_clone} id={index}/>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

#[function_component(ProjectsListPage)]
pub fn projects_page() -> Html {
    html! {
        <div>
            <NavBar/>
            <ProjectList/>
            <Footer/>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectDetailProps {
    pub id: usize,
}

#[function_component(ProjectDetailPage)]
pub fn project_detail_page(props: &ProjectDetailProps) -> Html {
    let projects = load_all_projects();
    let id = props.id;

    let selected_project = projects.get(id);
    html! {
        <div>
            <NavBar/>
            <section class="section">
                <div class="container">
                    <div class="content">
                        {
                        match selected_project {
                            Some(project) => {
                            let date_string = project.date.format("%B %d, %Y").to_string();
                            html!{
                                <div>
                                    <div class="columns">
                                        <div class="column">
                                            <h1 class="title title-bar">{project.title.clone()}</h1>
                                        </div>
                                        <div class="column is-narrow is-flex is-flex-direction-column is-justify-content-end">
                                            <h5 class="title-bar">{date_string}</h5>
                                        </div>
                                    </div>
                                    {project.full_page.clone()}
                                </div>
                            }
                        },
                        None => html!{<h1 align="center">{"Project not found :("}</h1>}
                        }
                        }
                    </div>
                </div>
            </section>
            <Footer/>
        </div>
    }
}
