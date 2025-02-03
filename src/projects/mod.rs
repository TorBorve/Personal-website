use chrono::NaiveDate;
use serde::Deserialize;
use yew::prelude::*;

mod car_mpc;

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
    let projects = vec![car_mpc::load()];

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
