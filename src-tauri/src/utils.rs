use tauri::api::dialog::blocking::FileDialogBuilder;

#[tauri::command]
pub async fn photos_library_dialog() -> String {
    let file_path = FileDialogBuilder::new().pick_file();
    match file_path {
        Some(pathbuf) => {
            match pathbuf.extension() {
                Some(ext) => {
                    if ext.eq("photoslibrary") {
                        pathbuf.to_str().unwrap().to_string()
                    } else {
                        "Try again".to_string()
                    }
                },
                None => "Nope".to_string()
            }
        },
        None => "No selection".to_string()
    }
}