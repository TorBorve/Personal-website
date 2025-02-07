use crate::projects::Project;

pub fn load() -> Project {
    load_markdown_project!(".")
}