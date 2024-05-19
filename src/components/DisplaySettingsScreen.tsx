import { Component, For, JSX, onMount } from "solid-js";
import { ModalProps, useModal } from "../modals/ModalProvider";
import { createStore } from "solid-js/store";
import GraphSettingsModal, { GraphStruct } from "../modals/GraphSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import GraphDisplayElement from "./SolidChart";
import ReadoutSettingsModal, { ReadoutStruct } from "../modals/ReadoutSettingsModal";
import ReadoutDisplayElement from "./Readout";
import { store } from "../core/file_handling";

/**
 * general set of props to give each display settingsModal
 */
export type SettingsModalProps = {
    displayStruct: DisplayStruct,
    index: number,
}

/**
 * An object that identifies the name and corresponding packet for a display element, along with its settings modal and display element
 */
export type DisplayStruct = {
    displayName: string,
    packetID: number,
    type: string,
    settingsModal: number,
    displayElement: number,
}
export const settingsModalArray = [
    GraphSettingsModal as ((props: ModalProps<SettingsModalProps>) => JSX.Element), 
    ReadoutSettingsModal as ((props: ModalProps<SettingsModalProps>) => JSX.Element)];
export const displayArray = [
    GraphDisplayElement as (graph: DisplayStruct) => JSX.Element, 
    ReadoutDisplayElement as (graph: DisplayStruct) => JSX.Element];

export const [displays, setDisplays] = createStore<DisplayStruct[]>([]);
let graphCounter = 1;
let readoutCounter = 1;

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

        //safety check
        for (let displayString in importedDisplays){
            let display = importedDisplays[displayString];
            console.log(display);
            if (display.type === `Graph`){
                let graph = display as GraphStruct;
                if(graph.settingsModal !== 0 || graph.displayElement !== 0 || graph.x === undefined || graph.y === undefined || graph.colors === undefined){
                    importedDisplays.splice(importedDisplays.indexOf(display),1);
                }
            }
            else if (display.type === `Readout`){
                let read = display as ReadoutStruct;
                if(read.settingsModal !== 1 || read.displayElement !== 1 || read.fields === undefined){
                    importedDisplays.splice(importedDisplays.indexOf(display),1);
                }
            }
            else{
                console.log(importedDisplays.indexOf(display));
                importedDisplays.splice(importedDisplays.indexOf(display),1);
            }
            console.log(importedDisplays);
        }
        setDisplays(importedDisplays);
    });

    return (
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2">
            {/*Field Select Button*/}
            <button class="m-1" onClick={() => {   
                if (PacketStructureViewModels.length != 0){
                    setDisplays([...displays, {
                        displayName: `Graph ${graphCounter}`, 
                        packetID: PacketStructureViewModels[0].id, 
                        type: `Graph`,
                        settingsModal: 0,
                        displayElement: 0,
                        x: 0, 
                        y: [0], 
                        colors: ["#FFD700", "#0000FF", "#000000", "#FF0000", "#00FF00"], 
                    } as GraphStruct]);
                    graphCounter++;
                    store.set("display", displays);
                }
            }}>
                New Graph
            </button>

            <button class="m-1" onclick={() => {
                if (PacketStructureViewModels.length !== 0){
                    setDisplays([...displays, {
                        displayName: `Readout ${readoutCounter}`,
                        packetID: PacketStructureViewModels[0].id,
                        type: `Readout`,
                        fields: [],
                        settingsModal: 1,
                        displayElement: 1,
                    } as unknown as ReadoutStruct]);
                    readoutCounter++;
                    store.set("display", displays);
                }
            }}>
                New Readout
            </button>

            {/*Fields*/}
            <div
                class="absolute grid flex-wrap top-10 bottom-8 left-0 right-0 m-a p-4 items-center justify-center gap-4 overflow-y-scroll"
                style={{ "width": "90%", "grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, displays.length)}, 1fr)`}}>
                <For each={displays}>
                    {(display: DisplayStruct, index) => {
                        return (
                            <div class="bg-stone-400 dark:bg-dark-900 flex justify-center items-center h-[100px] p-1.5 overflow-hidden rounded-7">
                                <button 
                                    class = "bg-white w-[100%] h-[100%] rounded-5.5 border-none justify-center dark:bg-dark-300"
                                    onClick={() => showModal<SettingsModalProps, {}>(settingsModalArray[display.settingsModal]  ?? 0, {
                                        displayStruct: display,
                                        index:index(),
                                    })
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
