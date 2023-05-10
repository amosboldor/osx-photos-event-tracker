import { createSignal, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

export default function SelectLibrary() {
  const [photosLibraryPath, setPhotosLibraryPath] = createSignal("Select a Photos Library");
  const [errorMessage, setErrorMessage] = createSignal("");

  async function photosLibraryDialog() {
    try {
      setPhotosLibraryPath(await invoke("photos_library_dialog"));
      setErrorMessage("");
    } catch (e) {
      setErrorMessage(e);
    }
  }

  return (
    <div class="p-4 flex flex-col">
      <div class="transition ease-in-out delay-150 hover:-translate-y-1 hover:scale-110 duration-300 border-4 shadow-2xl border-slate-600 rounded-lg flex w-max self-center mb-2">
        <button
          class="flex-none mx-auto bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-l-sm"
          type="button"
          onClick={() => photosLibraryDialog()}>
          Select Photos Library
        </button>
        <div class="grow p-2">{photosLibraryPath()}</div>
      </div>
      <Show when={errorMessage()}>
        <div class="bg-red-100 text-red-700 p-3 rounded-lg w-max self-center">
          <span class="font-bold">Error: </span>
          <span>{errorMessage()}</span>
        </div>
      </Show>



      <span class="text-red"></span>
    </div>
  );
}