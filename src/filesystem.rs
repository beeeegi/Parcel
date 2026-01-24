use crate::structures::*;
use log::{error, info, warn};
use serde::{ser::SerializeMap, Serialize, Serializer};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

const SRC: &str = "src";

fn serialize_project_tree<S: Serializer>(
    tree: &BTreeMap<String, TreePartition>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(tree.len() + 1))?;
    map.serialize_entry("$className", "DataModel")?;
    for (k, v) in tree {
        map.serialize_entry(k, v)?;
    }
    map.end()
}

#[derive(Clone, Debug, Serialize)]
struct Project {
    name: String,
    #[serde(serialize_with = "serialize_project_tree")]
    tree: BTreeMap<String, TreePartition>,
}

impl Project {
    fn new() -> Self {
        Self {
            name: "project".to_string(),
            tree: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileSystem {
    project: Project,
    root: PathBuf,
    source: PathBuf,
    errors: Vec<String>,
}

impl FileSystem {
    pub fn from_root(root: PathBuf) -> Self {
        let source = root.join(SRC);
        let project = Project::new();

        // Create source directory, log if it fails
        if let Err(e) = fs::create_dir_all(&source) {
            warn!("Could not create source directory: {}", e);
        }

        Self {
            project,
            root,
            source,
            errors: Vec::new(),
        }
    }

    /// Returns any errors that occurred during processing
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    /// Returns true if any errors occurred
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl InstructionReader for FileSystem {
    fn read_instruction<'a>(&mut self, instruction: Instruction<'a>) {
        match instruction {
            Instruction::AddToTree {
                name,
                mut partition,
            } => {
                // Check for duplicates but don't panic - log and skip
                if self.project.tree.get(&name).is_some() {
                    let msg = format!(
                        "Duplicate item in tree (skipping): {}",
                        name
                    );
                    warn!("{}", msg);
                    self.errors.push(msg);
                    return;
                }

                if let Some(path) = partition.path {
                    partition.path = Some(PathBuf::from(SRC).join(path));
                }

                for child in partition.children.values_mut() {
                    if let Some(path) = &child.path {
                        child.path = Some(PathBuf::from(SRC).join(path));
                    }
                }

                self.project.tree.insert(name, partition);
            }

            Instruction::CreateFile { filename, contents } => {
                let full_path = self.source.join(&filename);
                
                match File::create(&full_path) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(&contents) {
                            let msg = format!(
                                "Failed to write to file {:?}: {}",
                                filename, e
                            );
                            error!("{}", msg);
                            self.errors.push(msg);
                        }
                    }
                    Err(e) => {
                        let msg = format!(
                            "Failed to create file {:?}: {}",
                            filename, e
                        );
                        error!("{}", msg);
                        self.errors.push(msg);
                    }
                }
            }

            Instruction::CreateFolder { folder } => {
                let full_path = self.source.join(&folder);
                
                if let Err(e) = fs::create_dir_all(&full_path) {
                    let msg = format!(
                        "Failed to create folder {:?}: {}",
                        folder, e
                    );
                    error!("{}", msg);
                    self.errors.push(msg);
                }
            }
        }
    }

    fn finish_instructions(&mut self) {
        let project_path = self.root.join("default.project.json");
        
        // Serialize the project
        let json = match serde_json::to_string_pretty(&self.project) {
            Ok(json) => json,
            Err(e) => {
                let msg = format!("Failed to serialize project: {}", e);
                error!("{}", msg);
                self.errors.push(msg);
                return;
            }
        };

        // Create and write the file
        match File::create(&project_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json.as_bytes()) {
                    let msg = format!(
                        "Failed to write project file: {}",
                        e
                    );
                    error!("{}", msg);
                    self.errors.push(msg);
                } else {
                    info!("Created default.project.json");
                }
            }
            Err(e) => {
                let msg = format!(
                    "Failed to create project file: {}",
                    e
                );
                error!("{}", msg);
                self.errors.push(msg);
            }
        }
    }
}
