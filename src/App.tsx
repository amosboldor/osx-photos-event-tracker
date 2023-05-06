import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [photoPath, setPhotoPath] = createSignal("");

  async function photosLibraryDialog() {
    setPhotoPath(await invoke("photos_library_dialog"));
  }

  return (
    <div class="text-center">
      <button type="button" onClick={() => photosLibraryDialog()}>
        Select Photos Library
      </button>
      <p>{photoPath()}</p>
    </div>
  );
}

export default App;
