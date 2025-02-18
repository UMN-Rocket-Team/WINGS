import { Component, For, JSX, onMount } from "solid-js";
import { useParams } from "@solidjs/router";
import { DisplayStruct, displayArray, loadDisplays, displays } from "../components/DisplaySettingsScreen";
import { createStore } from "solid-js/store";
import { store } from "../core/file_handling";
import { GraphStruct } from "../modals/GraphSettingsModal";
import { ReadoutStruct } from "../modals/ReadoutSettingsModal";

const DisplayTab: Component = (): JSX.Element => {
    const params = useParams();
    // const [displays, setDisplays] = createStore<DisplayStruct[]>([]);
    onMount(loadDisplays);

    console.log(displays.length);
    return (
        <div class="flex flex-col flex-grow gap-4 rounded-lg dark:text-white">
            {/* Views */}
            <div class="grid gap-2 h-full" style={{ "grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, displays.length)}, 1fr)` }}>
                <For each={displays.filter(d => d.displayID === params.id)}>
                    {(display: DisplayStruct) => {
                        console.log(display.displayName)
                        let DisplayElement = displayArray[display.displayElement ?? 0];
                        return (
                            <div class="relative" style={{ height: '40vh' }}>
                                <DisplayElement {...display} />
                            </div>
                        );
                    }}
                </For>
            </div>
        </div>
    );
};

export default DisplayTab;
