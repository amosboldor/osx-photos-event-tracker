import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [photoPath, setPhotoPath] = createSignal("No Library Selected");

  async function photosLibraryDialog() {
    setPhotoPath(await invoke("photos_library_dialog"));
  }

  return (
    <div class="p-4">
      <div class="border-4 border-slate-600 rounded-lg flex w-max">
        <button class="flex-none mx-auto bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-l-sm" type="button" onClick={() => photosLibraryDialog()}>
          Select Photos Library
        </button>
        <div class="grow p-2">{photoPath()}</div>
      </div>
    </div>
  );
}

export default App;
