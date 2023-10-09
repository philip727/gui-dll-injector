import { Outlet } from "@solidjs/router";
import { appWindow } from "@tauri-apps/api/window";

const Toolbar = () => {
    return (
        <div class="w-screen h-screen bg-[var(--background)] border border-solid border-[var(--highlight)]">
            <div data-tauri-drag-region class="absolute top-0 left-0 w-screen h-4 flex flex-row justify-end items-center pr-1">
                <button onClick={() => appWindow.close()} class="text-[var(--highlight)] font-extralight" >
                    <img class="h-2 w-2" src="icons/cross.svg" />
                </button>
            </div>
            <Outlet />
        </div>
    )
}

export default Toolbar;
