import { invoke } from "@tauri-apps/api/tauri";
import { Process, processStore, setProcessStore } from "../store";
import { For, Match, Switch, onMount } from "solid-js";
import { appWindow } from "@tauri-apps/api/window";
import { truncateString } from "../scripts/scripts";

const MAX_LENGTH = 15;

const Proclist = () => {
    onMount(() => {
        invoke("get_all_processes").then(data => {
            setProcessStore("list", data as Process[]);
            setProcessStore("shownProcesses", data as Process[]);
        })
    })

    return (
        <div class="mt-6 px-8 pb-4 flex flex-col justify-start items-center">
            <input
                type="text"
                class="w-full h-8 outline-none bg-[var(--input)] pl-2 text-sm font-light rounded-sm"
                placeholder="search"
                onKeyUp={onSearch}
            />
            <div class="overflow-y-auto bg-[var(--input)] w-full h-64 flex flex-col overflow-x-hidden mt-3 py-1 rounded-sm">
                <For each={processStore.shownProcesses}>{data => (
                    <Switch fallback={
                        <button
                            onClick={() => {
                                setProcessStore("process", data);
                            }}
                            class="flex flex-row pl-2 py-1 hover:bg-[var(--input-hover)]"
                        >
                            <h1 class="text-sm font-extralight w-32 text-left">
                                {truncateString(data.name, MAX_LENGTH)}
                            </h1>
                            <h1 class="text-sm font-extralight text-left">
                                {data.pid}
                            </h1>
                        </button>
                    }>
                        <Match when={data.pid == processStore.process.pid}>
                            <button
                                class="flex flex-row pl-2 py-1 hover:bg-[var(--highlight-hover)] bg-[var(--highlight)]"
                            >
                                <h1 class="text-sm font-extralight w-32 text-left">
                                    {truncateString(data.name, MAX_LENGTH)}
                                </h1>
                                <h1 class="text-sm font-extralight text-left">
                                    {data.pid}
                                </h1>
                            </button>
                        </Match>
                    </Switch>
                )}
                </For>
            </div>
            <button
                onClick={() => setProcess()}
                class="bg-[var(--highlight)] w-full h-8 px-2 rounded-sm mt-4"
            >
                <h1 class="text-sm">set process</h1>
            </button>
        </div>
    )
}

const onSearch = (e: KeyboardEvent & { currentTarget: HTMLInputElement, target: Element }) => {
    let processList: Process[] = [];

    processStore.list.forEach(proc => {
        if (proc.name.toLowerCase().includes(e.currentTarget.value.toLowerCase())) {
            processList.push(proc);

        }
    })

    setProcessStore("shownProcesses", processList);
}

const setProcess = () => {
    if (processStore.process.name == "" && processStore.process.pid === null) {
        return;
    }

    invoke("set_selected_process", { name: processStore.process.name, pid: processStore.process.pid })
    appWindow.close()
}

export default Proclist;
