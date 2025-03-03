import { Component, For, JSX } from "solid-js";
import { useParams } from "@solidjs/router";
import { DisplaysContextValue, useDisplays } from "./DisplaysProvider";
import { displayRegistry, DisplayStruct } from "../core/display_registry";

const DisplayTab: Component = (): JSX.Element => {
    const params = useParams();
    const { displays }: DisplaysContextValue = useDisplays();

    return (
        <div class="flex flex-col flex-grow gap-4 rounded-lg dark:text-white">
            {/* Views */}
            <div class="grid gap-2 h-full" style={{ "grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, displays.length)}, 1fr)` }}>
                <For each={displays}>
                    {(display: DisplayStruct) => {
                        console.log(display.displayID);
                        const typeDef = displayRegistry.get(display.type)!;
                        const DisplayComponent = typeDef?.displayComponent;

                        return (
                            <div class="relative" style={{ height: '40vh' }}>
                                <DisplayComponent {...display} />
                            </div>
                        );
                    }}
                </For>
            </div>
        </div>
    );
};

export default DisplayTab;
