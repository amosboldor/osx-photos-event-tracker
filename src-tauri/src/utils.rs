use tauri::api::dialog::blocking::FileDialogBuilder;
use std::sync::Mutex;

pub struct AppState {
    pub photos_library_path: Mutex<String>
}

#[tauri::command]
pub async fn photos_library_dialog(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let pick_file_val = FileDialogBuilder::new().pick_file();
    if let Some(pathbuf) = pick_file_val {
        let is_photos_library = pathbuf.extension().map_or(false, |ext| ext.eq("photoslibrary"));
        if is_photos_library {
            let path_string = pathbuf.to_str().unwrap();
            *state.photos_library_path.lock().unwrap() = path_string.to_string();
            Ok(path_string.to_string())
        } else {
            Err("Not a Photos Library".into())
        }
    } else {
        Err("Need a Photos Library".into())
    }
}