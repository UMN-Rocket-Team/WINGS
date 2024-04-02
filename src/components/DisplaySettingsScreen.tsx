import { Component, For, JSX, createSignal } from "solid-js";
import { ModalProps, useModal } from "../modals/ModalProvider";
import { createStore, produce } from "solid-js/store";
import FieldSelectModal, { GraphModalProps, GraphStruct } from "../modals/GraphSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import closeIcon from "../assets/close.svg";
import SolidChart from "./SolidChart";
import VidSettingsModal from "../modals/GraphSettingsModal copy";
import { invoke } from "@tauri-apps/api";

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
    settingsModal: (props: ModalProps<SettingsModalProps>) => JSX.Element,
    displayElement: (graph: DisplayStruct) => JSX.Element,
}

/**
 * a store of all displays currently on the frontend
 */
export const [displays, setDisplays] = createStore<DisplayStruct[]>([]);

let counter = 1;
let subway = 0;
let family = 0;
let buttons = 0;

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

    return (
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2">
            {/*Field Select Button*/}
            <button onClick={() => 
            {   
                if (PacketStructureViewModels.length != 0){
                    setDisplays([...displays, {
                        displayName: `Graph ${counter}`, 
                        packetID: PacketStructureViewModels[0].id, 
                        settingsModal: FieldSelectModal,
                        displayElement: SolidChart,
                        x: 0, 
                        y: [0], 
                        colors: ["#FFD700", "#0000FF", "#000000", "#FF0000", "#00FF00"], 
                    } as GraphStruct]);
                    {counter = counter + 1};
                }
            }}>
                New Graph
            </button>

            <button onClick={() => {
                const vids = [
                    'https://www.youtube.com/embed/7ghSziUQnhs?autoplay=1&controls=0&start=20',
                    'https://www.youtube.com/embed/4GZRICFNeT0?autoplay=1&controls=0&start=48'
                ];
                const url = vids[(subway++) % vids.length];
                setDisplays([...displays, {
                    displayName: `Subway Surfers ${subway}`,
                    packetID: PacketStructureViewModels[0].id,
                    settingsModal: VidSettingsModal,
                    displayElement: () => (
                        <iframe credentialless anonymous class="w-100% h-100%" src={url} frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
                    )
                }]);
            }}>
                New Subway Surfers
            </button>

            <button onClick={() => {
                const vids = [
                    'https://www.youtube.com/embed/e94a9nB3N_c?autoplay=1&controls=0&start=20',
                    'https://www.youtube.com/embed/ROIk-Fv8S0M?autoplay=1&controls=0&start=48'
                ];
                const url = vids[(family++) % vids.length];
                setDisplays([...displays, {
                    displayName: `Family Guy Funny Moments ${family}`,
                    packetID: PacketStructureViewModels[0].id,
                    settingsModal: VidSettingsModal,
                    displayElement: () => (
                        <iframe credentialless anonymous class="w-100% h-100%" src={url} frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
                    )
                }]);
            }}>
                New Family Guy Funny Moments
            </button>

            <button onClick={() => {
                const initial = 5;
                let [seconds, setSeconds] = createSignal(initial);
                const tick = () => {
                    if (seconds() > 0) {
                        setSeconds(seconds() - 1);
                    } else {
                        clearInterval(int);
                        invoke('button_goofy');
                    }
                }
                let int = setInterval(tick, 1000);
                setDisplays([...displays, {
                    displayName: `Button ${++buttons}`,
                    packetID: 1,
                    settingsModal: () => <h1>Settings</h1>,
                    displayElement: () => (
                        <div class="text-center w-100% h-100%">
                            <h1>{seconds()}</h1>
                            <button class="w-120px h-40px rounded" style={{
                                "background-color": seconds() >= 52 ? 'purple'
                                    : seconds() >= 42 ? 'blue'
                                    : seconds() >= 32 ? 'green'
                                    : seconds() >= 22 ? 'yellow'
                                    : seconds() >= 12 ? 'orange'
                                    : 'red'
                            }} onClick={() => {
                                setSeconds(initial);
                                clearInterval(int);
                                int = setInterval(tick, 1000);
                            }}></button>
                        </div>
                    )
                }]);
            }}>
                New The Button
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
                                    onClick={() => showModal<SettingsModalProps, {}>(display.settingsModal, {
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
