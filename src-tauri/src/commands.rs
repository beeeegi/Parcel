use crate::logging::{LogEntry, TauriLogger};
use log::info;
use parcel_lib::filesystem::FileSystem;
use parcel_lib::process_instructions;
use serde::Serialize;
use std::borrow::Cow;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use tauri_plugin_dialog::DialogExt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Invalid file extension. Only .rbxl and .rbxlx files are supported.")]
    InvalidFileExtension,

    #[error("Failed to open file: {0}")]
    FileOpenError(String),

    #[error("Failed to decode file: {0}")]
    DecodeError(String),

    #[error("Failed to create output directory: {0}")]
    DirectoryCreateError(String),

    #[error("Conversion task failed: {0}")]
    TaskError(String),
}

impl serde::Serialize for ConversionError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Serialize)]
pub struct ConversionResult {
    pub success: bool,
    pub message: String,
    pub output_path: Option<String>,
}

#[tauri::command]
pub async fn select_output_folder(app: tauri::AppHandle) -> Result<Option<String>, ConversionError> {
    let folder = app.dialog()
        .file()
        .set_title("Select Output Folder")
        .blocking_pick_folder();

    match folder {
        Some(path) => Ok(Some(path.to_string())),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn select_input_file(app: tauri::AppHandle) -> Result<Option<String>, ConversionError> {
    let file = app.dialog()
        .file()
        .set_title("Select Roblox Place File")
        .add_filter("Roblox Place Files", &["rbxl", "rbxlx"])
        .blocking_pick_file();

    match file {
        Some(path) => {
            let path_str = path.to_string();
            if path_str.ends_with(".rbxl") || path_str.ends_with(".rbxlx") {
                Ok(Some(path_str))
            } else {
                Err(ConversionError::InvalidFileExtension)
            }
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn run_conversion(
    input_path: String,
    output_folder: String,
) -> Result<ConversionResult, ConversionError> {
    info!("Starting conversion...");
    info!("Input file: {}", input_path);
    info!("Output folder: {}", output_folder);

    let result = tokio::task::spawn_blocking(move || {
        do_conversion(&input_path, &output_folder)
    })
    .await
    .map_err(|e| ConversionError::TaskError(e.to_string()))?;

    result
}

fn do_conversion(input_path: &str, output_folder: &str) -> Result<ConversionResult, ConversionError> {
    let file_path = PathBuf::from(input_path);
    
    let extension = file_path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase());
    
    match extension.as_deref() {
        Some("rbxl") | Some("rbxlx") => {}
        _ => {
            info!("Error: Invalid file extension");
            return Err(ConversionError::InvalidFileExtension);
        }
    }

    info!("Opening place file...");
    let file = fs::File::open(&file_path)
        .map_err(|e| ConversionError::FileOpenError(e.to_string()))?;
    let file_source = BufReader::new(file);

    info!("Decoding place file (this may take a moment for large files)...");
    let tree = match file_path
        .extension()
        .map(|e| e.to_string_lossy())
    {
        Some(Cow::Borrowed("rbxlx")) => {
            rbx_xml::from_reader_default(file_source)
                .map_err(|e| ConversionError::DecodeError(format!(
                    "XML decode error: {}. Try saving the file as .rbxl (binary format) in Roblox Studio instead.", 
                    e
                )))
        }
        Some(Cow::Borrowed("rbxl")) => {
            rbx_binary::from_reader_default(file_source)
                .map_err(|e| ConversionError::DecodeError(format!("Binary decode error: {}", e)))
        }
        _ => Err(ConversionError::InvalidFileExtension),
    }?;

    let file_stem = file_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());
    
    let output_path = PathBuf::from(output_folder).join(&file_stem);
    
    info!("Creating output directory: {}", output_path.display());
    fs::create_dir_all(&output_path)
        .map_err(|e| ConversionError::DirectoryCreateError(e.to_string()))?;

    info!("Processing Roblox instances...");
    let mut filesystem = FileSystem::from_root(output_path.clone());
    process_instructions(&tree, &mut filesystem);

    info!("Conversion completed successfully!");
    info!("Output saved to: {}", output_path.display());

    Ok(ConversionResult {
        success: true,
        message: format!("Successfully converted to {}", output_path.display()),
        output_path: Some(output_path.to_string_lossy().to_string()),
    })
}

#[tauri::command]
pub fn get_logs() -> Vec<LogEntry> {
    TauriLogger::get_logs()
}

#[tauri::command]
pub fn clear_logs() {
    TauriLogger::clear_logs();
}

#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
