import { invoke } from '@tauri-apps/api'
import './main.css'
import { Component, onCleanup, onMount } from "solid-js"
import { dllStore, processStore, setDllStore, setProcessStore } from '../store'
import { UnlistenFn, listen } from '@tauri-apps/api/event'
import { open as openF } from '@tauri-apps/api/dialog';
import { truncateString } from '../scripts/scripts'
import DisplayBox from '../layout/DisplayBox'

const MAX_PATH_DISPLAY = 30;
const Main: Component = () => {
    let unlisten!: UnlistenFn;

    onMount(async () => {
        unlisten = await listen("selected_pid", (event) => {
            const payload = event.payload as { name: string, pid: number };
            setProcessStore("process", { name: payload.name, pid: payload.pid })
        })
    })

    onCleanup(() => {
        unlisten();
    })

    return (
        <div class="flex flex-col h-full mt-6 gap-4">
            <div class="flex justify-center items-center gap-4">
                <DisplayBox
                    text={`${processStore.process.name} ${processStore.process.pid == null ? `${""}` : `(${processStore.process.pid})`} `}
                />
                <button onClick={() => { invoke("open_process_list") }} class="bg-[var(--highlight)] w-32 h-8 px-2 rounded-sm ">
                    <h1 class="text-sm">select process</h1>
                </button>
            </div>
            <div class="flex justify-center items-center gap-4">
                <DisplayBox
                    text={truncateString(dllStore.file, MAX_PATH_DISPLAY)}
                />
                <button
                    onClick={() => openFile()}
                    class="bg-[var(--highlight)] w-32 h-8 px-2 rounded-sm "
                >
                    <h1 class="text-sm">select dll</h1>
                </button>
            </div>
        </div>
    )
}

const openFile = async () => {
    const selected = await openF({
        multiple: false,
        filters: [{
            name: "DLL",
            extensions: ["dll"]
        }]
    })

    setDllStore("file", selected as string)
}

export default Main;
