
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::{NavBar, Footer};
use crate::routes::Route;
use crate::projects::{load_all_projects, Project};


#[derive(Properties, PartialEq, Clone)]
struct ProjectCardProps {
    project: Project,
    id: usize,
}

#[function_component(ProjectCard)]
fn project_card(props: &ProjectCardProps) -> Html {
    let ProjectCardProps{project, id} = props.clone();

    let formated_date = project.date.format("%B %d, %Y").to_string();

    html! {
        <Link<Route> to={Route::ProjectDetail {id: id}}>
        <div class="project-card">
            <div class="columns">
                <div class="column is-one-third">
                    <figure class="image is-1by1">
                        <img src={project.image_url.clone()} alt={format!("Image of {}", project.title)}/>
                    </figure>
                </div>
                <div class="column">
                    <h1 class="title is-size-2">{&project.title}</h1>
                    <h2 class="subtitle is-size-3">{&project.summary}</h2>
                    <p class="is-size-5" style="
                        position: absolute;
                        bottom: 0;
                        right: 0;
                        padding: 10px 20px;
                    ">{formated_date}</p>
                </div>
            </div>
        </div>
        </Link<Route>>
    }
}

#[function_component(ProjectList)]
fn project_list() -> Html {
    let projects= load_all_projects();

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
    let projects= load_all_projects();
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
                            Some(project) => html!{
                                project.full_page.clone()
                        },
                        None => html!{<h1>{"Project not found :("}</h1>}
                        }
                        }
                    </div>
                </div>
            </section>
            <Footer/>
        </div>
    }

}