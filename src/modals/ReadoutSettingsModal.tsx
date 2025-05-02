import { ModalProps } from "../core/ModalProvider";
import DefaultModalLayout from "../core/DefaultModalLayout";
import { For, JSX, Show, createSignal, onMount } from "solid-js";
import {SettingsModalProps, displays, setDisplays } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { createStore, produce } from "solid-js/store";
import infoIcon from "../assets/info-sym.svg";
import dropdownIcon from "../assets/dropdown.svg";
import { store } from "../core/file_handling";
import { DisplayStruct } from "../core/display_registry";

interface ReadoutStructField {
    // index of field in packet
    packetFieldIndex: number;
    unit: string;
}
export class ReadoutStruct implements DisplayStruct {
    displayName = `Readout`;
    packetID = -1;
    type = `readout`;
    fields: ReadoutStructField[] = [];
    packetsDisplayed: boolean[] = [false];
}

const ReadoutSettingsModal = (props: ModalProps<SettingsModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

    // used to restore previous name when user enters something invalid
    let oldName = props.displayStruct.displayName;

    const [displayInfo, setDisplayInfo] = createSignal(false); // Is info about the display being displayed?

    const [displayStruct, setDisplayStruct] = createStore(props.displayStruct as ReadoutStruct);
    
    let infoIconRef: HTMLImageElement | undefined;
    onMount(() => { // Events for hovering over info icon
        infoIconRef?.addEventListener("mouseout", (e) => {
            setDisplayInfo(false);
        });
        infoIconRef?.addEventListener("mouseover", (e) => {
            setDisplayInfo(true);
        });
    });

    /** handleInput will handle updating the graphs name and also catches blank inputs and reverts to previous name */
    const handleInput = (event: Event) => {
        const newName = (event.target as HTMLElement).textContent || '';
        if (newName.trim() !== '') {
            setDisplayName(newName.trim(), props.index);
            oldName = newName.trim();
        } else {
            (event.target as HTMLElement).textContent = oldName;
        }
    };

    /* handleKeyDown helps handle updating the graphName by preventing enters(newlines) */
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

    const getStructField = (packetId: number, fieldIndex: number): ReadoutStructField | undefined => {
        if (props.displayStruct.packetID !== packetId) {
            return undefined;
        }
        return displayStruct.fields.find(i => i.packetFieldIndex === fieldIndex);
    };

    const setActive = (packetId: number, fieldIndex: number, active: boolean) => {
        setDisplays(produce(s => {
            const struct = s[props.index] as ReadoutStruct;

            // When switching packet IDs, remove all the old stuff
            if (struct.packetID !== packetId) {
                struct.packetID = packetId;
                struct.fields = [];
            }

            if (active) {
                struct.fields.push({
                    packetFieldIndex: fieldIndex,
                    unit: ''
                });
            } else {
                struct.fields = struct.fields.filter(i => i.packetFieldIndex !== fieldIndex);
            }
        }));
        store.set("display", displays);
    };

    return <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
        <div class="flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg relative min-w-fit">
            <Show when={displayInfo()}>
                <div class="absolute bg-gray-200 top-[-1px] left-[-1px] dark:bg-neutral-700 p-4 rounded-3xl pt-12 z-[2]">
                    <p class="max-w-prose">Displays incoming data for chosen variables.</p>
                </div>            
            </Show>
            
            <div class='flex flex-row leading-none justify-between mb-4'>
                <img alt="Info" src={infoIcon} ref={infoIconRef} draggable={false} class="relative top-0 w-[23px] dark:invert z-[3]" />

                <h3 contenteditable={true} class="m-2 text-center font-bold w-[82%] absolute left-[50%] translate-x-[-50%]" 
                    onBlur={handleInput} onKeyDown={handleKeyDown}>
                    {props.displayStruct.displayName}
                </h3>
            </div>

            <For each={PacketStructureViewModels}>{(packetViewModel, packetIdx) => (
                <div class='flex flex-col mb-4'>
                    <div class='flex gap-2 leading-none w-fit cursor-pointer'
                        onClick={() => {
                            setDisplays(produce(s => {
                                const struct = (s[props.index] as ReadoutStruct);
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

                    <Show when={displays[props.index]?.packetsDisplayed[packetIdx()]}>
                        <div class='flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg'>
                            <For each={packetViewModel.components.filter(i => i.type === PacketComponentType.Field)}>{(component) => {
                                const packetField = component.data as PacketField;
                                const structField = () => getStructField(packetViewModel.id, packetField.index);
                                return <label class="flex flex-row cursor-pointer">
                                    <input
                                        type="checkbox"
                                        class="mr-1 cursor-pointer"
                                        checked={!!structField()}
                                        onChange={(e) => {
                                            const target = e.target as HTMLInputElement;
                                            setActive(packetViewModel.id, packetField.index, target.checked);
                                        }}
                                    />

                                    {packetField.name}

                                    <Show when={structField()}>
                                        <input
                                            type="text"
                                            value={structField()!.unit}
                                            class="ml-2"
                                            onChange={(e) => {
                                                const target = e.target as HTMLInputElement;
                                                setDisplays(produce(s => {
                                                    const struct = s[props.index] as ReadoutStruct;
                                                    const componentField = struct.fields.find(i => i.packetFieldIndex === packetField.index);
                                                    if (componentField) {
                                                        componentField.unit = target.value;
                                                    }
                                                }));
                                                store.set("display", displays);
                                            }}
                                        />
                                    </Show>
                                </label>
                            }}</For>
                        </div>
                    </Show>
                </div>
            )}</For>
        </div>
    </DefaultModalLayout>;
};

export default ReadoutSettingsModal;