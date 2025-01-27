
use yew::prelude::*;
use yew_router::prelude::Link;

use chrono::NaiveDate;

use crate::components::{NavBar, Footer};
use crate::routes::Route;

#[derive(Clone, PartialEq)]
struct Project {
    id: usize,
    title: String,
    date: NaiveDate,
    image_url: String,
    summary: String,
    full_page: Html,
}

#[derive(Properties, PartialEq, Clone)]
struct ProjectCardProps {
    project: Project,
}

#[function_component(ProjectCard)]
fn project_card(props: &ProjectCardProps) -> Html {
    let ProjectCardProps{project} = props.clone();

    let formated_date = project.date.format("%B %d, %Y").to_string();

    html! {
        <Link<Route> to={Route::ProjectDetail {id: project.id}}>
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
    let projects= vec![
        Project {
            id: 1,
            title: "Project 1".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 10, 30).unwrap(),
            image_url: "static/tor_verbier.jpg".to_string(),
            summary: "This was a good one!".to_string(),
            full_page: html! {
                <>
                    <h1 class="title">{"Project 1 Details"}</h1>
                    <p>{"Here is a detailed explanation of Project 1."}</p>
                    <ul>
                        <li>{"Feature 1: Amazing functionality"}</li>
                        <li>{"Feature 2: Easy to use"}</li>
                        <li>{"Feature 3: Open-source"}</li>
                    </ul>
                </>
            }
        },
        Project {
            id: 2,
            title: "Project 2".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            image_url: "static/auc_passed_technical.jpg".to_string(),
            summary: "This was a good two!".to_string(),
            full_page: html! {
                <>
                    <h1 class="title">{"Project 2 Details"}</h1>
                    <p>{"Here is a detailed explanation of Project 2."}</p>
                    <ul>
                        <li>{"Feature 1: Amazing functionality"}</li>
                        <li>{"Feature 2: Easy to use"}</li>
                        <li>{"Feature 3: Open-source"}</li>
                    </ul>
                </>
            }
        },
    ];

    html! {
        <div class="container">
            <div class="columns is-multiline">
                {for projects.into_iter().map(|project| {
                    let project_clone = project.clone();
                    html!{
                        <div class="column is-full">
                            <ProjectCard project={project_clone}/>
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
    let projects= vec![
        Project {
            id: 1,
            title: "Project 1".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 10, 30).unwrap(),
            image_url: "static/tor_verbier.jpg".to_string(),
            summary: "This was a good one!".to_string(),
            full_page: html! {
                <>
                    <h1 class="title">{"Project 1 Details"}</h1>
                    <p>{"Here is a detailed explanation of Project 1."}</p>
                    <ul>
                        <li>{"Feature 1: Amazing functionality"}</li>
                        <li>{"Feature 2: Easy to use"}</li>
                        <li>{"Feature 3: Open-source"}</li>
                    </ul>
                </>
            }
        },
        Project {
            id: 2,
            title: "Project 2".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            image_url: "static/auc_passed_technical.jpg".to_string(),
            summary: "This was a good two!".to_string(),
            full_page: html! {
                <>
                    <h1 class="title">{"Project 2 Details"}</h1>
                    <p>{"Here is a detailed explanation of Project 2."}</p>
                    <ul>
                        <li>{"Feature 1: Amazing functionality"}</li>
                        <li>{"Feature 2: Easy to use"}</li>
                        <li>{"Feature 3: Open-source"}</li>
                    </ul>
                </>
            }
        },
    ];
    let id = props.id;

    let selected_project = projects.iter().find(|&project| project.id == id);
    html! {
        <div>
            <NavBar/>
            {
            match selected_project {
                Some(project) => html!{
                    project.full_page.clone()
            },
            None => html!{<p> {"Project not found :("}</p>}
            }
            }
            <Footer/>
        </div>
    }

}