import { Component } from "solid-js";

export type DisplayBoxProps = {
    text: string
}

const DisplayBox: Component<DisplayBoxProps> = (props) => {
    return (
        <div class="bg-[var(--input)] h-8 w-64 flex items-center justify-start pl-2 rounded-sm">
            <h1 class="text-sm">
                {props.text}
            </h1>
        </div>
    )
}

export default DisplayBox;
