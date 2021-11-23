use daw_ui::models::Project;

fn create_project(project_path: &str) {
    let project_index_path = format!("{}/index.flexbuf", project_path);
    log::info!("Writing project to {}", project_index_path);
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        title: "New project".to_string(),
    };
    let project = flexbuffers::to_vec(project).unwrap();
    std::fs::write(project_index_path, project).unwrap();
}

fn main() {
    wisual_logger::init_from_env();

    create_project(&format!(
        "{}/examples/example.dawproject",
        env!("CARGO_MANIFEST_DIR")
    ));
}
