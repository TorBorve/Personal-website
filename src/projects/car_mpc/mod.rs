
use crate::projects::{Project, ProjectMetaData, convert_markdown_to_html};
use chrono::NaiveDate;


pub fn load() -> Project {
    let metadata = include_str!("metadata.json");
    let metadata: ProjectMetaData = serde_json::from_str(&metadata).expect("Failed to parse metadata json file");
    let md_text = include_str!("content.md");
    let md_html = convert_markdown_to_html(md_text);
    let date = NaiveDate::parse_from_str(&metadata.date, "%Y-%m-%d").expect("Invalid date in metadata file");
    Project {
        title: metadata.title,
        date: date,
        image_url: metadata.image_url,
        summary: metadata.summary,
        full_page: md_html,
    }
}
