use chrono::NaiveDate;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize, Debug)]
struct ProjectMetaData {
    title: String,
    date: String,
    image_url: String,
    summary: String,
}

#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub date: NaiveDate,
    pub image_url: String,
    pub summary: String,
    pub full_page: Html,
}

pub fn load_all_projects() -> Vec<Project> {
    let mut projects = vec![
        car_mpc::load(),
        website::load(),
        control_systems_torbox::load(),
    ];

    projects.sort_by(|a, b| b.date.partial_cmp(&a.date).unwrap());
    projects
}

fn convert_markdown_to_html(markdown_text: &str) -> Html {
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(markdown_text, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    let html_parsed = Html::from_html_unchecked(AttrValue::from(html_output));
    html! {
        <div>
            {html_parsed}
        </div>
    }
}

#[macro_export]
macro_rules! load_markdown_project {
    ($folder:expr) => {{
        let metadata = include_str!(concat!($folder, "/metadata.json"));
        let metadata: $crate::projects::ProjectMetaData =
            serde_json::from_str(metadata)
                .expect("Failed to parse metadata json file");
        let md_text = include_str!(concat!($folder, "/content.md"));
        let md_html = $crate::projects::convert_markdown_to_html(md_text);
        let date =
            chrono::NaiveDate::parse_from_str(&metadata.date, "%Y-%m-%d")
                .expect("Invalid date in metadata file");
        Project {
            title: metadata.title,
            date,
            image_url: metadata.image_url,
            summary: metadata.summary,
            full_page: md_html,
        }
    }};
}

mod car_mpc;
mod control_systems_torbox;
mod website;
