#[macro_use]
extern crate rocket;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use lcov_parser::LCOVRecord;
use rocket::http::{ContentType, MediaType, Status};
use rocket::State;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
struct Function {
    name: String,
    line: u32,
}

#[derive(Debug, PartialEq, Serialize)]
struct LineEntry {
    line: u32,
    count: u32,
}

#[derive(Debug, PartialEq, Default, Serialize)]
struct SourceFile {
    path: PathBuf,
    functions: Vec<Function>,
    coverage_lines: Vec<LineEntry>,
}

#[derive(Debug, Serialize)]
struct CoverageData {
    files: HashMap<PathBuf, SourceFile>,
    contents: HashMap<PathBuf, String>,
}

#[derive(Serialize)]
struct SourceLine {
    index: u32,
    line: String,
    covered: &'static str,
    entry: LineEntry,
}

#[derive(Serialize)]
struct ContentsLineModel {
    lines: Vec<SourceLine>,
}

impl ContentsLineModel {
    fn build(source_file: &SourceFile, contents: &str) -> Self {
        rust_code_analysis::RustCode::
        let num_lines = contents.lines().count();
        let mut entries_by_line = Vec::with_capacity(num_lines);
        entries_by_line.resize(num_lines, 0);
        for entry in &source_file.coverage_lines {
            entries_by_line[entry.line as usize] = entry.count;
        }

        let lines = contents.lines();
        let lines = lines
            .enumerate()
            .map(|(index, line)| SourceLine {
                index: index as u32,
                line: line.to_string(),
                covered: if entries_by_line[index] > 0 {
                    "covered"
                } else {
                    "not-covered"
                },
                entry: LineEntry {
                    line: index as u32,
                    count: entries_by_line[index],
                },
            })
            .collect();
        Self { lines }
    }
}

#[get("/source_files")]
fn get_source_files(state: &State<CoverageData>) -> (Status, (ContentType, Template)) {
    let state = state.inner();

    (
        Status::Ok,
        (
            ContentType(MediaType::HTML),
            Template::render("source_files", state),
        ),
    )
}

#[get("/source_files/<path..>")]
fn get_source_file(
    state: &State<CoverageData>,
    path: PathBuf,
) -> (Status, (ContentType, Option<Template>)) {
    let state = state.inner();

    let source_file = state.files.get(&path);
    let contents = state.contents.get(&path);

    let context = source_file
        .zip(contents)
        .map(|(source_file, contents)| Context {
            contents: ContentsLineModel::build(source_file, contents),
            source_file,
        });

    #[derive(Serialize)]
    struct Context<'a> {
        source_file: &'a SourceFile,
        contents: ContentsLineModel,
    }

    if let Some(context) = context {
        (
            Status::Ok,
            (
                ContentType(MediaType::HTML),
                Some(Template::render("source_file", context)),
            ),
        )
    } else {
        (Status::NotFound, (ContentType(MediaType::Text), None))
    }
}

#[launch]
fn app() -> _ {
    let lcov_contents = std::fs::read_to_string("./lcov.info").unwrap();
    let result = lcov_parser::parse_report(&*lcov_contents).unwrap();

    let mut source_files: HashMap<PathBuf, SourceFile> = HashMap::default();
    let mut current_source_file = SourceFile::default();

    let root_directory = Path::new("../../../../");

    for record in result {
        match &record {
            LCOVRecord::SourceFile(file_name) => {
                let path = fix_path(file_name);
                current_source_file.path = path;
            }
            LCOVRecord::FunctionName(function) => current_source_file.functions.push(Function {
                name: function.name.clone(),
                line: function.line,
            }),
            LCOVRecord::FunctionData(function_data) => {
                function_data.
            }
            LCOVRecord::Data(line_data) => current_source_file.coverage_lines.push(LineEntry {
                line: line_data.line,
                count: line_data.count,
            }),
            LCOVRecord::EndOfRecord => {
                source_files.insert(
                    current_source_file.path.clone(),
                    std::mem::take(&mut current_source_file),
                );
            }
            _ => {}
        }
    }

    let contents: HashMap<PathBuf, String> = source_files
        .iter()
        .map(|(path, _source_file)| {
            let file_path = root_directory.join(path);
            (path.clone(), std::fs::read_to_string(file_path).unwrap())
        })
        .collect();

    rocket::build()
        .manage(CoverageData {
            contents,
            files: source_files,
        })
        .attach(Template::fairing())
        .mount("/", routes![get_source_files, get_source_file])
}

fn fix_path(source_file: &str) -> PathBuf {
    let path = Path::new(source_file);
    let path = path
        .strip_prefix("/home/runner/work/augmented-audio/augmented-audio/")
        .unwrap();
    path.into()
}
