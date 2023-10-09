import { createStore } from 'solid-js/store'

export type ProcessStore = {
    process: Process,
    list: Process[],
    shownProcesses: Process[]
}

export type DllStore = {
    file: string,
}

export type Process = {
    name: string,
    pid: number  | null,
}

export const [processStore, setProcessStore] = createStore<ProcessStore>({
    process: { name: "", pid: null },
    list: [],
    shownProcesses: []
})

export const [dllStore, setDllStore] = createStore<DllStore>({
    file: ""
})
