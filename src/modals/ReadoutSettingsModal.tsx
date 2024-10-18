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

export interface ReadoutModalProps extends SettingsModalProps {
    displayStruct: ReadoutStruct;
}

interface ReadoutStructField {
    // index of field in packet
    packetFieldIndex: number;
    unit: string;
}
export interface ReadoutStruct extends DisplayStruct {
    fields: ReadoutStructField[];
}

const ReadoutSettingsModal = (props: ModalProps<ReadoutModalProps>): JSX.Element => {
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

    // used to restore previous name when user enters something invalid
    let oldName = props.displayStruct.displayName;

    const getStructField = (packetId: number, fieldIndex: number): ReadoutStructField | undefined => {
        if (props.displayStruct.packetID !== packetId) {
            return undefined;
        }
        return props.displayStruct.fields.find(i => i.packetFieldIndex === fieldIndex);
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

    const [displayInfo, setDisplayInfo] = createSignal(false);

    return <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">

        <Show when={displayInfo()}>
            <div class="absolute bg-neutral-300 dark:bg-neutral-700 p-4 top-16 rounded-3xl right-1 p-t-10 p-r-0 z-1">
                Display the latest value from a group of variable in the same packet.
            </div>          
        </Show>

        <img alt="Info" src={infoIcon} ref={infoIconRef} draggable={false} class="relative w-[6%] dark:invert z-2" />

        <div class="flex flex-col bg-neutral-200 rounded-10 dark:bg-gray p-2">
            <h2>
                <input
                    value={props.displayStruct.displayName}
                    class="text-lg border-0 p-0 m-0 bg-transparent text-center font-bold"
                    onChange={(e) => {
                        setDisplays(produce(s => {
                            const struct = s[props.index] as ReadoutStruct;
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
                                    type="text"
                                    value={structField()!.unit}
                                    onchange={(e) => {
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
                </>
            )}</For>
        </div>

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
    </DefaultModalLayout>;
};

export default ReadoutSettingsModal;