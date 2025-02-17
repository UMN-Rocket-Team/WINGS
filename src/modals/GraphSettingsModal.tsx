import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX, createSignal, Show, onMount } from "solid-js";
import { DisplayStruct, SettingsModalProps } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponent, PacketComponentType, PacketField, PacketStructureViewModel } from "../backend_interop/types";
import settingsIcon from "../assets/settings.png";
import infoIcon from "../assets/info-sym.svg";
import dropdownIcon from "../assets/dropdown.svg"
import { produce } from "solid-js/store";
import { store } from "../core/file_handling";
import { DisplaysContextValue, useDisplays } from "../components/DisplaysProvider";


/**
 * generic interface for all g
 */
export interface GraphModalProps extends SettingsModalProps {
    /** Graph that is being passed */
    displayStruct: GraphStruct;
    /** Index of graph so that handleSelect[Y/X] can be called correctly! */
}
export interface GraphStruct extends DisplayStruct {
    x: number; //fieldIndex
    y: number[];
    colors: string[];
}

/**
 * A modal component that allows a user to modify the fields contained in a screen.
 * 
 * @param props an object that contains a function to close the modal, the list of fields that are selected, and a callback to select a field
 */
const GraphSettingsModal = (props: ModalProps<GraphModalProps>): JSX.Element => {
    const { displays, setDisplays }: DisplaysContextValue = useDisplays();
    const { PacketStructureViewModels } = useBackend();

    /** Signal used to help handleInput revert from blank inputs to most recent name */
    const [graphCurrName, setName] = createSignal(props.displayStruct.displayName);


    const [displaySettings, setDisplaySettings] = createSignal(false); // Are the modal settings being displayed?
    const [displayInfo, setDisplayInfo] = createSignal(false); // Is info about the display being displayed?

    let infoIconRef: HTMLImageElement | undefined;
    onMount(() => { // Events for hovering over info icon
        infoIconRef?.addEventListener("mouseout", () => {
            setDisplayInfo(false);
        });
        infoIconRef?.addEventListener("mouseover", () => {
            setDisplayInfo(true);
        });
    });

    /** handleInput will handle updating the graphs name and also catches blank inputs and reverts to previous name */
    const handleInput = (event: Event) => {
        const newName = (event.target as HTMLElement).textContent || '';
        if (newName.trim() !== '') {
            setGraphName(newName.trim(), props.index);
            setName(newName.trim());
        } else {
            (event.target as HTMLElement).textContent = graphCurrName();
        }
    };

    /* handleKeyDown helps handle updating the graphName by preventing enters(newlines) */
    const handleKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'Enter') {
            event.preventDefault();
        }
    };

    const handleSelectY = (isChecked: boolean, fieldIndex: number, graphIndex: number, packet_id: number) => {
        if (isChecked) {
            setDisplays(produce((s) => {
                if (s[graphIndex].packetID != packet_id) {
                    (s[graphIndex] as GraphStruct).y = [];
                    s[graphIndex].packetID = packet_id;
                    (s[graphIndex] as GraphStruct).x = 0; //sets x back to 0 to avoid overflow problems
                }
                (s[graphIndex] as GraphStruct).y.push(fieldIndex);
            }));
        } else {
            setDisplays(produce((s) =>
                (s[graphIndex] as GraphStruct).y = (s[graphIndex] as GraphStruct).y.filter(ind => ind != fieldIndex)));
        }
        store.set("display", displays);
    }

    const handleSelectX = (isChecked: boolean, fieldIndex: number, graphIndex: number, packet_id: number) => {
        if (isChecked) {
            setDisplays(produce((s) => {
                if (s[graphIndex].packetID != packet_id) {
                    (s[graphIndex] as GraphStruct).y = (s[graphIndex] as GraphStruct).y.filter(_ => false); //sets all y values to false
                    s[graphIndex].packetID = packet_id;
                }
                (s[graphIndex] as GraphStruct).x = fieldIndex;
            }));
        } else {
            setDisplays(produce((s) =>
                (s[graphIndex] as GraphStruct).x = 0));
        }
        store.set("display", displays);
    }

    const setGraphName = (newName: string, index: number) => {
        setDisplays(produce((s) =>
            s[index].displayName = newName));
        store.set("display", displays);
    }

    const deleteGraph = (index: number) => {
        let newGraphs: DisplayStruct[] = [];
        for (let i = 0; i < displays.length; i++) {
            if (index !== i) {
                newGraphs.push(displays[i]);
            }
        }
        setDisplays(newGraphs);
        store.set("display", displays);
    }

    const updateColor = (color: string, colorIndex: number, graphIndex: number) => {
        setDisplays(produce((s) =>
            (s[graphIndex] as GraphStruct).colors[colorIndex] = color));
        store.set("display", displays);
    }

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">

            <div class='flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg relative min-w-fit'>
                <Show when={displayInfo()}>
                    <div class="absolute bg-neutral-300 top-[-1px] left-[-1px] dark:bg-neutral-700 p-4 rounded-3xl pt-12 z-[2]">
                        Customizable graph for visualizing data.
                    </div>            
                </Show>

                <div class='flex flex-row leading-none justify-between mb-4'>
                    <img alt="Info" src={infoIcon} ref={infoIconRef} draggable={false} class="relative top-0 w-[23px] dark:invert z-[3]" />

                    <h3 contenteditable={true} class="m-2 text-center font-bold w-[82%] absolute left-[50%] translate-x-[-50%]" 
                        onBlur={handleInput} onKeyDown={handleKeyDown}>
                        {graphCurrName()}
                    </h3>

                    <img alt="Settings" src={settingsIcon} draggable={false} onClick={() => setDisplaySettings(s => !s)} 
                        class="relative top-0 w-[25px] dark:invert z-[1] cursor-pointer" />
                </div>

                <Show when={displaySettings()}>
                    <div class="absolute bg-neutral-300 dark:bg-neutral-700 p-4 top-0 rounded-3xl right-0 z-[0]">
                        {/* Graph Colors */}
                        <div class="flex flex-col bg-neutral-300 dark:bg-neutral-700 p-4 text-center">
                            <h2 class="font-bold">Graph Colors</h2>
                            <For each={PacketStructureViewModels.find(psViewModel => psViewModel.id === props.displayStruct.packetID)?.components.filter(component => component.type === PacketComponentType.Field)}>
                                {(packetComponent: PacketComponent, i) => {
                                    const field = packetComponent.data as PacketField;
                                    return (
                                        <label class="flex items-center justify-center space-x-2">
                                            {field.name}
                                            <input type="color" class="rounded-full" value={props.displayStruct.colors[i() % props.displayStruct.colors.length]} onInput={(event) => {
                                                updateColor((event.target as HTMLInputElement).value, i(), props.index);
                                            }} />
                                        </label>
                                    );
                                }}
                            </For>
                        </div>
                        <div class="relative flex items-center justify-center mt-2">
                            <button
                                class="rounded-lg bg-red-500 hover:bg-red-600 flex items-center justify-center p-3"
                                onClick={() => {
                                    deleteGraph(props.index);
                                    props.closeModal({});
                                }}>
                                <h3>Remove Display</h3>
                            </button>
                        </div>
                    </div>
                </Show>

                <For each={PacketStructureViewModels}>
                    {(PacketStructureViewModel: PacketStructureViewModel, packetIdx) =>
                        <div class='flex flex-col mb-4'>
                            <div class='flex gap-2 leading-none w-fit cursor-pointer'
                                onClick={() => {
                                    setDisplays(produce(s => {
                                        const struct = (s[props.index] as GraphStruct);
                                        struct.packetsDisplayed[packetIdx()] = !struct.packetsDisplayed[packetIdx()];
                                    }));
                                    store.set("display", displays);
                                }}>
                                <img alt="Dropdown" src={dropdownIcon} 
                                    class={`h-4 dark:invert`} 
                                    style={`transform: rotate(${displays[props.index]?.packetsDisplayed[packetIdx()] ? "0deg" : "270deg"});`}
                                    draggable={false}/>
                                <h3 class='font-bold'>{PacketStructureViewModel.name}</h3>
                            </div>

                            <Show when={displays[props.index]?.packetsDisplayed[packetIdx()]}>
                                <div class='flex bg-neutral-200 dark:bg-gray-700 p-4 pt-0 pb-0 rounded-lg'>
                                    <div class='flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg'>
                                        <h2 class="font-bold">X-Axis</h2>
                                        <For each={PacketStructureViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                            {(packetComponent: PacketComponent) => {
                                                const field = packetComponent.data as PacketField;
                                                return (
                                                    <label class="flex items-center space-x-2">
                                                        <input type="radio"
                                                            class="form-radio"
                                                            checked={props.displayStruct.x === field.index && props.displayStruct.packetID === PacketStructureViewModel.id} // Check based on the state
                                                            onClick={(event) =>
                                                                handleSelectX((event.target as HTMLInputElement).checked, field.index, props.index, PacketStructureViewModel.id)
                                                            }
                                                        />
                                                        {field.name}
                                                    </label>
                                                );
                                            }}
                                        </For>
                                    </div>
                                    
                                    <div class='flex flex-col bg-neutral-200 dark:bg-gray-700 p-4'>
                                        <h2 class="font-bold">Y-Axis</h2>
                                        <For each={PacketStructureViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                            {(packetComponent: PacketComponent) => {
                                                const field = packetComponent.data as PacketField;
                                                return (
                                                    <label class="flex items-center space-x-2">
                                                        <input type="checkbox"
                                                            class="form-checkbox"
                                                            checked={props.displayStruct.y.some(selectedField => selectedField === field.index) && props.displayStruct.packetID === PacketStructureViewModel.id}
                                                            onClick={(event) => {
                                                                handleSelectY((event.target as HTMLInputElement).checked, field.index, props.index, PacketStructureViewModel.id);
                                                            }} />
                                                        {field.name}
                                                    </label>
                                                );
                                            }}
                                        </For>
                                    </div>
                                </div>                                
                            </Show>
                        </div>
                    }
                </For>
            </div>
        </DefaultModalLayout>
    );
};

export default GraphSettingsModal;