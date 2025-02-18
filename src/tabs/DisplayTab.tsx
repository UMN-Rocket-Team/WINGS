import { Component, For, JSX } from "solid-js";
import { DisplayStruct, displayArray } from "../components/DisplaySettingsScreen";
import GraphDisplayElement from "../components/SolidChart";
import ReadoutDisplayElement from "../components/Readout";
import { DisplaysContextValue, useDisplays } from "../components/DisplaysProvider";

const DisplayTab: Component = (): JSX.Element => {
    const { displays }: DisplaysContextValue = useDisplays();

    return (
        <div class="flex flex-col flex-grow gap-4 rounded-lg dark:text-white">
            {/* Views */}
            <div class="grid gap-2 h-full" style={{ "grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, displays.length)}, 1fr)` }}>
                <For each={displays}>
                    {(display: DisplayStruct) => {
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
