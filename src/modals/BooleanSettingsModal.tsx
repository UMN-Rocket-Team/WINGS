import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX, Show } from "solid-js";
import { DisplayStruct, SettingsModalProps, displays, setDisplays } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { produce } from "solid-js/store";
import closeIcon from "../assets/close.svg";
import { store } from "../core/file_handling";

export interface BooleanSettingsModalProps extends SettingsModalProps {
    displayStruct: BooleanStruct;
}

interface BooleanStructField {
    // index of field in packet
    packetFieldIndex: number;
    unit: string;
}

export interface BooleanStruct extends DisplayStruct {
    fields: BooleanStructField[];
}

const BooleanSettingsModal = (props: ModalProps<BooleanSettingsModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

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
                    unit: ''
                });
            } else {
                struct.fields = struct.fields.filter(i => i.packetFieldIndex !== fieldIndex);
            }
        }));
        store.set("display", displays);
    };

    return <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
        <div class="flex items-center justify-center">
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
                        return <label class="flex flex-row">
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
                                <input
                                    type="number"
                                    value={structField()!.unit}
                                    onchange={(e) => {
                                        const target = e.target as HTMLInputElement;
                                        setDisplays(produce(s => {
                                            const struct = s[props.index] as BooleanStruct;
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
                </>
            )}</For>
        </div>

        {/* <div class="flex items-center justify-center">
            <button
                class="w-[10%] h-[10%] rounded-5 border-none text-center"
                onClick={() => {
                    setDisplays(displays.filter((graph, index) => index !== props.index));
                    store.set("display", displays);
                    props.closeModal({});
                }}>
                <img alt="Delete" src={closeIcon} class="w-full h-full dark:invert justify-center" draggable={false} />
            </button>
        </div> */}
    </DefaultModalLayout>;

    
};

export default BooleanSettingsModal;