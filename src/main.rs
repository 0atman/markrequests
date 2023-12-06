use git2::Repository;
use std::path::Path;

mod prelude {
    #![allow(unused_imports)]
    // https://docs.rs/itertools/latest/itertools/
    pub use itertools::Itertools;
    // https://docs.rs/std_prelude/latest/std_prelude/
    pub use std_prelude::*;
    // https://docs.rs/maplit/latest/maplit/
    pub use maplit::*;
    // https://lib.rs/crates/serde
    pub use serde::{Deserialize, Serialize};
    // https://lib.rs/crates/colored
    pub use colored::*;
}
use prelude::*;

/// source: https://github.com/rust-lang/git2-rs/issues/996#issuecomment-1781360955
/// Get the contents of a file from a git repository
///
/// # Arguments
///
/// * `file` - The relative path to the file
/// * `branch` - The branch to pull the version from
/// * `repo` - The repository to pull the file from
///
/// # Errors
///
/// [git2::Error]
///
/// # Examples
///
/// ```
/// let repo = git2::Repository::open(".")?;
/// let content = get_file_content(".gitignore", "main", &repo)?;
/// ```
fn get_file_content(
    file: &str,
    branch: &str,
    repo: &Repository,
) -> Result<String, Box<dyn std::error::Error>> {
    let rev = repo.revparse_single(branch)?;
    let tree = rev.peel_to_tree()?;

    let path = tree.get_path(Path::new(file))?;

    let obj_path = path.to_object(repo)?;
    let blob = obj_path.into_blob().unwrap();

    let content = std::str::from_utf8(blob.content())?;

    Ok(content.to_string())
}

// fn get_folder_files(
//     folder: &str,
//     branch: &str,
//     repo: &Repository,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let rev = repo.revparse_single(branch)?;
//     let tree = rev.peel_to_tree()?;
//
//     let path = tree.get_path(Path::new(folder))?;
//
//     let obj_path = path.to_object(repo)?;
//     let blob = obj_path.into_blob().unwrap();
//
//     let content = std::str::from_utf8(blob.content())?;
//
//     Ok(content.to_string())
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Metadata {
    head: String,
    assignee: String,
    base: String,
    draft: bool,
    issue: Option<String>,
    reviewers: Vec<String>,
    tags: Vec<String>,
    title: Option<String>,
}

fn get_metadata(md_string: String) -> Metadata {
    let split_md = md_string.split("---\n").collect::<Vec<&str>>();
    let yaml_string = split_md[1];
    let mut document: Metadata = serde_yaml::from_str(yaml_string).unwrap();

    let parsed_md = markdown::to_mdast(split_md[2], &markdown::ParseOptions::default()).unwrap();

    let title = parsed_md.children().unwrap()[0].children().unwrap()[0].to_string();
    document.title = Some(title.clone());
    document
}

fn main() {
    let repo = Repository::open("/home/oatman/projects/demo-markrequests").unwrap();

    //dbg!(get_folder_files("prs/", "markrequests", &repo));

    let md_string = get_file_content("prs/example.md", "markrequests", &repo).unwrap();

    let document = get_metadata(md_string);
    println!(
        "{}",
        &document
            .title
            .clone()
            .unwrap()
            .italic()
            .bold()
            .bright_blue()
    );
    dbg!(&document);
}
