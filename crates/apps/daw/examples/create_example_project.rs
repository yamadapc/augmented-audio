// = copyright ====================================================================
// DAW: Flutter UI for a DAW application
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
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
