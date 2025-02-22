import { Component, For, JSX, onMount } from "solid-js";
import { ModalProps, useModal } from "../core/ModalProvider";
import { createStore, SetStoreFunction } from "solid-js/store";
import GraphSettingsModal, { GraphStruct } from "../modals/GraphSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import GraphDisplayElement from "./SolidChart";
import ReadoutSettingsModal, { ReadoutStruct } from "../modals/ReadoutSettingsModal";
import ReadoutDisplayElement from "./Readout";
import BooleanSettingsModal, { BooleanStruct } from "../modals/BooleanSettingsModal";
import Boolean from "./Boolean";
import { store } from "../core/file_handling";
import { displayRegistry, DisplayStruct, DisplayTypeDefinition } from "../core/display_registry";

/**
 * general set of props to give each display settingsModal
 */
export type SettingsModalProps = {
    displayStruct: DisplayStruct,
    index: number,
}

export const [displays, setDisplays] = createStore<DisplayStruct[]>([]);
let counter = 1;


/**
 * A component that:
 * - Displays a list of selected fields added to this screen
 * - Allows users to add fields to the screen
 * - Allows users to clear the screen
 * - Allows users to view the graphed data received for the selected fields
 *
 * @param props an object that contains the number of this screen
 */
const FieldsScreen: Component = () => {
    const { PacketStructureViewModels } = useBackend();
    const { showModal } = useModal();

    onMount(async () => {
        /**
         * a store of all displays currently on the frontend
         */
        let importedDisplays: DisplayStruct[] = await store.get("display") ?? [];

        //safety check to remove any non-expected display types
        for (let displayString in importedDisplays){
            let display = importedDisplays[displayString];
            display.packetsDisplayed = display.packetsDisplayed ?? []
            if (display.type === `Graph`){
                let graph = display as GraphStruct;
                if(graph.settingsModal !== 0 || graph.displayElement !== 0 || graph.x === undefined || graph.y === undefined || graph.colors === undefined){
                    importedDisplays.splice(importedDisplays.indexOf(display),1);
                }

            } else if (display.type === `Readout`){
                let read = display as ReadoutStruct;
                if(read.settingsModal !== 1 || read.displayElement !== 1 || read.fields === undefined){
                    importedDisplays.splice(importedDisplays.indexOf(display),1);
                }

            } else if (display.type === `Indicator`) {
                let read = display as ReadoutStruct;
                if(read.settingsModal !== 2 || read.displayElement !== 2 || read.fields === undefined){
                    importedDisplays.splice(importedDisplays.indexOf(display),1);
                }
            }
            else{
                console.log(importedDisplays.indexOf(display));
                importedDisplays.splice(importedDisplays.indexOf(display),1);
            }
        }
        setDisplays(importedDisplays);
    });

    return (
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2 mb-5">
            {/*Field Select Button*/}
            <div>
                {Array.from(displayRegistry.values()).map((typeDef) => (
                    <button type="button" class="m-1 text-black bg-gray-100 hover:bg-gray-200 focus:outline-none focus:ring-4
                    focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 z-1000
                    dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white" 
                    onClick={() => {
                        const newDisplay = new typeDef.structClass();
                        newDisplay.displayName = `${typeDef.displayName} ${counter}`;
                        newDisplay.packetID= PacketStructureViewModels[0].id;
                        setDisplays([...displays, newDisplay]);
                        counter++;
                        store.set("display", displays);
                    }}
                    >
                    New {typeDef.displayName}
                    </button>
                ))}
            </div>

            {/*Fields*/}
            <div
                class="absolute grid flex-wrap top-10 bottom-8 left-0 mt-5 right-0 m-auto p-4 items-center justify-center gap-4 overflow-y-scroll"
                style={{ "width": "90%", "grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, displays.length)}, 1fr)`}}>
                <For each={displays}>
                    {(display: DisplayStruct, index) => {
                        return (
                            <div class="bg-stone-400 dark:bg-neutral-900 flex justify-center items-center h-[100px] p-1.5 overflow-hidden rounded-2xl">
                                <button 
                                    class="bg-white w-full h-full rounded-[1.375rem] border-0 justify-center dark:bg-neutral-700"
                                    onClick={() => {showModal<SettingsModalProps, {}>(settingsModalArray[display.settingsModal] ?? 0, {
                                        displayStruct: display,
                                        index:index(),
                                    } as SettingsModalProps)}
                                }>
                                    <h3 class="text-black dark:text-white">{display.displayName}</h3>
                                </button>
                            </div>
                        );
                    }}
                </For>
            </div>
        </div>
    )
}

export default FieldsScreen;
