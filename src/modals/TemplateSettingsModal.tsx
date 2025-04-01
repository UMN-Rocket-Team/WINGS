import { ModalProps } from "../core/ModalProvider";
import DefaultModalLayout from "../core/DefaultModalLayout";
import { For, JSX, Show, createSignal, onMount } from "solid-js";
import {SettingsModalProps, displays, setDisplays } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { createStore, produce } from "solid-js/store";
import settingsIcon from "../assets/settings.png";
import infoIcon from "../assets/info-sym.svg";
import dropdownIcon from "../assets/dropdown.svg";
import { store } from "../core/file_handling";
import { DisplayStruct } from "../core/display_registry";


export class TemplateStruct implements DisplayStruct {

    // Implementing required values
    displayName = `Template`;
    packetID = -1;
    type = `template`;
    packetsDisplayed: boolean[] = [false];

    // Adding an array of all fields that will be displayed by this element
    fields: number[] = [];
}

// The modal that will be displayed to the user when editing a template type
const TemplateSettingsModal = (props: ModalProps<SettingsModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

    // Used to restore previous name when user enters something invalid
    let oldName = props.displayStruct.displayName;

    const [displaySettings, setDisplaySettings] = createSignal(false); // Are the modal settings (on the top left of the modal) being displayed?
    const [displayInfo, setDisplayInfo] = createSignal(false); // Is info about the display being displayed?


    const [displayStruct, setDisplayStruct] = createStore(props.displayStruct as TemplateStruct);
    
    let infoIconRef: HTMLImageElement | undefined;
    onMount(() => { // Events for hovering over info icon
        infoIconRef?.addEventListener("mouseout", (e) => {
            setDisplayInfo(false);
        });
        infoIconRef?.addEventListener("mouseover", (e) => {
            setDisplayInfo(true);
        });
    });

    /** HandleInput will handle updating the graphs name and also catches blank inputs and reverts to previous name */
    const handleInput = (event: Event) => {
        const newName = (event.target as HTMLElement).textContent || '';
        if (newName.trim() !== '') {
            setDisplayName(newName.trim(), props.index);
            oldName = newName.trim();
        } else {
            (event.target as HTMLElement).textContent = oldName;
        }
    };

    /* HandleKeyDown helps handle updating the graphName by preventing enters(newlines) */
    const handleKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'Enter') {
            event.preventDefault();
        }
    };

    const setDisplayName = (newName: string, index: number) => {
        setDisplays(produce(s => {
            s[index]!.displayName = newName;
        }));
        store.set("display", displays);
    }

    const deleteDisplay = () => {
        // Need to clear fields before removing display
        setDisplays(produce(s => {
            const struct = s[props.index] as TemplateStruct;
            struct.fields =[];
        }));
        store.set("display", displays);
        
        setDisplays(displays.filter((_, index) => index !== props.index));
        store.set("display", displays);
        props.closeModal({});
    }

    const getStructField = (packetId: number, fieldIndex: number): Number | undefined => {
        if (props.displayStruct.packetID !== packetId) {
            return undefined;
        }
        return displayStruct.fields.find(i => i === fieldIndex);
    };

    const setActive = (packetId: number, fieldIndex: number, active: boolean) => {
        setDisplays(produce(s => {
            const struct = s[props.index] as TemplateStruct;

            // When switching packet IDs, remove all the old stuff
            if (struct.packetID !== packetId) {
                struct.packetID = packetId;
                struct.fields = [];
            }

            if (active) {
                struct.fields.push(fieldIndex);
            } else {
                struct.fields = struct.fields.filter(i => i !== fieldIndex);
            }
        }));
        store.set("display", displays);
    };

    return <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
        <div class="flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg relative min-w-fit">

            {/*More info button*/}
            <Show when={displayInfo()}>
                <div class="absolute bg-neutral-300 top-[-1px] left-[-1px] dark:bg-neutral-700 p-4 rounded-3xl pt-12 z-[2]">
                    <p class="max-w-prose">This is a Template Settings Modal, and should not be shown to the user</p>
                </div>            
            </Show>
            
            {/*name of the display, along with the Extra settings button*/}
            <div class='flex flex-row leading-none justify-between mb-4'>
                <img alt="Info" src={infoIcon} ref={infoIconRef} draggable={false} class="relative top-0 w-[23px] dark:invert z-[3]" />

                <h3 contenteditable={true} class="m-2 text-center font-bold w-[82%] absolute left-[50%] translate-x-[-50%]" 
                    onBlur={handleInput} onKeyDown={handleKeyDown}>
                    {props.displayStruct.displayName}
                </h3>

                <img alt="Settings" src={settingsIcon} draggable={false} onClick={() => setDisplaySettings(s => !s)} 
                    class="relative top-0 w-[25px] dark:invert z-[1] cursor-pointer" />
            </div>

            {/*
            Extra settings, which always contain the button to delete the display, 
            they can also contain other settings that apply to the entire modal, like toggling a debug mode and 
            */}
            <Show when={displaySettings()}>
                <div class="absolute bg-neutral-300 dark:bg-neutral-700 p-4 top-0 rounded-3xl right-0 z-[0]">
                    <div class="relative flex items-center justify-center mt-10">
                        <button
                            class="rounded-lg bg-red-500 hover:bg-red-600 flex items-center justify-center p-3"
                            onClick={() => {
                                deleteDisplay();
                            }}>
                            <h3>Remove Display</h3>
                        </button>
                    </div>
                </div>
            </Show>

            {/* List of every single packet, each packet has a dropdown with its fields*/}
            <For each={PacketStructureViewModels}>{(packetViewModel, packetIdx) => (
                <div class='flex flex-col mb-4'>
                    <div class='flex gap-2 leading-none w-fit cursor-pointer'
                        onClick={() => {
                            setDisplays(produce(s => {
                                const struct = (s[props.index] as TemplateStruct);
                                struct.packetsDisplayed[packetIdx()] = !struct.packetsDisplayed[packetIdx()];
                            }));
                            store.set("display", displays);
                        }}>
                        <img alt="Dropdown" src={dropdownIcon} 
                            class={`h-4 dark:invert`} 
                            style={`transform: rotate(${displays[props.index]?.packetsDisplayed[packetIdx()] ? "0deg" : "270deg"});`}
                            draggable={false}/>
                        <h3 class='font-bold'>{packetViewModel.name}</h3>
                    </div>

                    {/* Checks the packets displayed array and renders the opened dropdown for each displayed packet*/}
                    <Show when={displays[props.index]?.packetsDisplayed[packetIdx()]}>
                        <div class='flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg'>
                            <For each={packetViewModel.components.filter(i => i.type === PacketComponentType.Field)}>{(component) => {
                                const packetField = component.data as PacketField;
                                const structField = () => getStructField(packetViewModel.id, packetField.index);
                                return <label class="flex flex-row cursor-pointer">
                                    {/**
                                     * Lets the user select specific packets for use,.
                                     * If there are settings for each packet on the screen (like the ability to rename a field or assign it a color), 
                                     * they should also be edited from here
                                     */}
                                    <input
                                        type="checkbox"
                                        class="mr-1 cursor-pointer"
                                        checked={!!structField()}
                                        onchange={(e) => {
                                            const target = e.target as HTMLInputElement;
                                            setActive(packetViewModel.id, packetField.index, target.checked);
                                        }}
                                    />
                                    {packetField.name}
                                </label>
                            }}</For>
                        </div>
                    </Show>
                </div>
            )}</For>
        </div>
    </DefaultModalLayout>;
};

export default TemplateSettingsModal;