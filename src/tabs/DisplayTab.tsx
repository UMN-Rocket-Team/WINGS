import { Component, For, JSX, Show } from "solid-js";
import { displays, FlexviewObject, flexviewObjects } from "../components/DisplaySettingsScreen";
import GraphDisplayElement from "../components/SolidChart";
import ReadoutDisplayElement from "../components/Readout";
import { displayRegistry, DisplayStruct, DisplayTypeDefinition } from "../core/display_registry";

const RecursiveFlexviewViewer = (props: {
    object: FlexviewObject
}) => {
    if (props.object.type === 'display') {
        const display = props.object;
        const typeDef = displayRegistry.get(displays[display.index].type)!;
        const DisplayComponent = typeDef?.displayComponent;
        return (
            <div
                class="overflow-hidden w-full h-full flex flex-shrink items-center justify-center border-2 border-gray p-2"
            >
                <DisplayComponent {...displays[display.index]} />
            </div>
        );
    }

    if (props.object.type === 'layout') {
        const layout = props.object;
        return (
            <div
                class="overflow-hidden w-full h-full flex items-center justify-center border-2 border-gray p-2 gap-2"
                style={{
                    "flex-direction": layout.direction
                }}
            >
                <Show
                    when={layout.children.length > 0}
                    fallback={(
                        <p>Empty layout</p>
                    )}
                >
                    <For each={layout.children}>{(childObjectId, childObjectIndex) => <>
                        <div
                            class = {"flex-shrink"} 
                            style={layout.direction === 'column' ? {
                                width: '100%',
                                height: `${layout.weights[childObjectIndex()] * 100}%`
                            } : {
                                width: `${layout.weights[childObjectIndex()] * 100}%`,
                                height: '100%'
                            }}
                        >
                            <RecursiveFlexviewViewer
                                object={flexviewObjects[childObjectId]}
                            />
                        </div>
                    </>}</For>
                </Show>
            </div>
        );
    }

    return (
        <div>Unknown flexview object</div>
    );
};

const DisplayTab: Component = (): JSX.Element => {
    return (
        <div class="flex flex-col flex-grow flex-shrink gap-4 rounded-lg dark:text-white">
            {/* Views */}
            {/* <div class="grid gap-2 h-full" style={{ "grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, displays.length)}, 1fr)` }}>
                <For each={displays}>
                    {(display: DisplayStruct) => {
                        const typeDef = displayRegistry.get(display.type)!;
                        const DisplayComponent = typeDef?.displayComponent;
                        
                        return (
                        <div class="relative" style={{ height: '40vh' }}>
                            <DisplayComponent {...display} />
                        </div>
                        );
                    }}
                </For>
            </div> */}
            <RecursiveFlexviewViewer
                object={flexviewObjects[0]}
            />
        </div>
    );
};

export default DisplayTab;
