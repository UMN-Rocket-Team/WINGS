import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX, Show, createSignal, onMount } from "solid-js";
import { DisplayStruct, SettingsModalProps, displays, setDisplays } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { produce } from "solid-js/store";
import closeIcon from "../assets/close.svg";
import infoIcon from "../assets/info-sym.svg";
import { store } from "../core/file_handling";

export interface BooleanSettingsModalProps extends SettingsModalProps {
    displayStruct: BooleanStruct;
}

interface BooleanStructField {
    // index of field in packet
    packetFieldIndex: number;
    unit: {left: string; right: string};
    sign: string;
    isRange: boolean;
}

export interface BooleanStruct extends DisplayStruct {
    fields: BooleanStructField[];
}

const BooleanSettingsModal = (props: ModalProps<BooleanSettingsModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

    let infoIconRef: HTMLImageElement | undefined;
    onMount(() => { // Events for hovering over info icon
        infoIconRef?.addEventListener("mouseout", (e) => {
            setDisplayInfo(false);
            console.log(displayInfo());
        });
        infoIconRef?.addEventListener("mouseover", (e) => {
            setDisplayInfo(true);
            console.log(displayInfo());
        });
    });

    const signs = ["<", "=", ">"];

    // used to restore previous name when user enters something invalid
    let oldName = props.displayStruct.displayName;

    const getStructField = (packetId: number, fieldIndex: number): BooleanStructField | undefined => {
        if (props.displayStruct.packetID !== packetId) {
            return undefined;
        }
        return props.displayStruct.fields.find(i => i.packetFieldIndex === fieldIndex);
    };

    const setActive = (packetId: number, fieldIndex: number, active: boolean) => {
        setDisplays(produce(s => {
            const struct = s[props.index] as BooleanStruct;

            // When switching packet IDs, remove all the old stuff
            if (struct.packetID !== packetId) {
                struct.packetID = packetId;
                struct.fields = [];
            }

            if (active) {
                struct.fields.push({
                    packetFieldIndex: fieldIndex,
                    unit: {left: '', right: ''},
                    sign: "<",
                    isRange: false
                });
            } else {
                struct.fields = struct.fields.filter(i => i.packetFieldIndex !== fieldIndex);
            }
        }));
        store.set("display", displays);
    };

    const [displayInfo, setDisplayInfo] = createSignal(false);

    return <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">

        <Show when={displayInfo()}>
            <div class="absolute bg-neutral-300 dark:bg-neutral-700 p-4 top-21 rounded-3xl right-1 p-t-10 p-r-0 z-1">
                Monitors chosen variables and displays lights that indicate whether data satisfies inputted inequalities or not.
            </div>            
        </Show>

        <img alt="Info" src={infoIcon} ref={infoIconRef} draggable={false} class="relative top-5 w-[6%] dark:invert z-2" />


        <div class="flex items-center justify-center z-0">
            <button
                class="w-[10%] h-[10%] rounded-5 border-none text-center"
                onClick={() => {
                    setDisplays(displays.filter((graph, index) => index !== props.index));
                    store.set("display", displays);
                    props.closeModal({});
                }}>
                <img alt="Delete" src={closeIcon} class="w-full h-full dark:invert justify-center" draggable={false} />
            </button>
        </div>

        <div class="flex flex-col bg-neutral-200 rounded-10 dark:bg-gray p-2">
            <h2>
                <input
                    value={props.displayStruct.displayName}
                    class="text-lg border-0 p-0 m-0 bg-transparent text-center font-bold"
                    onChange={(e) => {
                        setDisplays(produce(s => {
                            const struct = s[props.index] as BooleanStruct;
                            const value = (e.target as HTMLInputElement).value.trim();
                            if (value) {
                                struct.displayName = value;
                                oldName = value;
                            } else {
                                struct.displayName = oldName;
                            }
                        }));
                        store.set("display", displays);
                    }}
                />
            </h2>

            <For each={PacketStructureViewModels}>{(packetViewModel, index) => (
                <>
                    <h3>
                        {packetViewModel.name}
                    </h3>

                    <For each={packetViewModel.components.filter(i => i.type === PacketComponentType.Field)}>{(component, componentIndex) => {
                        const packetField = component.data as PacketField;
                        const structField = () => getStructField(packetViewModel.id, packetField.index);
                        const getComponentField = () => {
                            const struct = (displays[props.index] as BooleanStruct);
                            const componentField = struct.fields.find(i => i.packetFieldIndex === packetField.index);
                            return componentField;
                        }
                        
                        return <label class="flex flex-row justify-center">
                            <Show when={structField()}>
                                <Show when={getComponentField()?.isRange}>
                                    <input
                                        type="number"
                                        value={structField()!.unit?.left}
                                        class="w-12"
                                        onchange={(e) => {
                                            const target = e.target as HTMLInputElement;
                                            setDisplays(produce(s => {
                                                const struct = (s[props.index] as BooleanStruct);
                                                const componentField = struct.fields.find(i => i.packetFieldIndex === packetField.index);
                                                if (componentField) {
                                                    componentField.unit = {...componentField.unit, left: target.value};
                                                }
                                            }));
                                            store.set("display", displays);
                                        }}
                                    />   

                                    <select value={getComponentField()?.sign} class="m-r-1" onInput={(e) => {
                                        const target = e.target as HTMLInputElement;
                                        setDisplays(produce(s => {
                                            const struct = (s[props.index] as BooleanStruct);
                                            const componentField = struct.fields.find(i => i.packetFieldIndex === packetField.index);
                                            if (componentField) {
                                                componentField.sign = target.value;
                                            }
                                        }));    
                                        store.set("display", displays);                               
                                    }}>
                                        <option value="<">{"<"}</option>
                                    </select>  
                                </Show>                                
                            </Show>

                            <input
                                type="checkbox"
                                checked={!!structField()}
                                onchange={(e) => {
                                    const target = e.target as HTMLInputElement;
                                    setActive(packetViewModel.id, packetField.index, target.checked);
                                }}
                            />

                            {packetField.name}

                            <Show when={structField()}>
                                <select value={getComponentField()?.sign} class="m-l-2" onInput={(e) => {
                                    const target = e.target as HTMLInputElement;
                                    setDisplays(produce(s => {
                                        const struct = (s[props.index] as BooleanStruct);
                                        const componentField = struct.fields.find(i => i.packetFieldIndex === packetField.index);
                                        if (componentField) {
                                            componentField.sign = target.value;
                                        }
                                    }));    
                                    store.set("display", displays);                               
                                }}>
                                    {!getComponentField()?.isRange ? 
                                        <For each={signs}>{(sign) =>{return <option value={sign}>{sign}</option>}}</For> : 
                                        <option value="<">{"<"}</option>}
                                </select>                                

                                <input
                                    type="number"
                                    value={structField()!.unit?.right}
                                    class="w-12"
                                    onchange={(e) => {
                                        const target = e.target as HTMLInputElement;
                                        setDisplays(produce(s => {
                                            const struct = (s[props.index] as BooleanStruct);
                                            const componentField = struct.fields.find(i => i.packetFieldIndex === packetField.index);
                                            if (componentField) {
                                                componentField.unit = {...componentField.unit, right: target.value};
                                            }
                                        }));
                                        store.set("display", displays);
                                    }}
                                />

                                <label for={`range-select-${packetField.index}`} class="m-l-2">
                                    Range?
                                    <input
                                        type="checkbox"
                                        id={`range-select-${packetField.index}`}
                                        checked={getComponentField()?.isRange}
                                        onchange={(e) => {
                                            const target = e.target as HTMLInputElement;
                                            setDisplays(produce(s => {
                                                const struct = (s[props.index] as BooleanStruct);
                                                const componentField = struct.fields.find(i => i.packetFieldIndex === packetField.index);
                                                if (componentField) {
                                                    componentField.isRange = target.checked;
                                                }
                                            }));
                                            store.set("display", displays);
                                        }}
                                    />
                                </label>
                            </Show>
                        </label>
                    }}</For>
                </>
            )}</For>
        </div>
    </DefaultModalLayout>;

    
};

export default BooleanSettingsModal;