import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX } from "solid-js";
import { DisplayStruct, SettingsModalProps, displays, setDisplays } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { produce } from "solid-js/store";
import closeIcon from "../assets/close.svg";

export interface ReadoutModalProps extends SettingsModalProps {
    displayStruct: ReadoutStruct;
}
export interface ReadoutStruct extends DisplayStruct {
    fields: Array<{
        // index of field in packet
        packetFieldIndex: number;
    }>;
}

const ReadoutSettingsModal = (props: ModalProps<ReadoutModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

    // used to restore previous name when user enters something invalid
    let oldName = props.displayStruct.displayName;

    const isActive = (packetId: number, fieldIndex: number): boolean => (
        props.displayStruct.packetID === packetId &&
        !!props.displayStruct.fields.find(i => i.packetFieldIndex === fieldIndex)
    );

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
                    packetFieldIndex: fieldIndex
                });
            } else {
                struct.fields = struct.fields.filter(i => i.packetFieldIndex !== fieldIndex);
            }
        }));
    };
 
    return <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
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
                        return <label class="flex flex-row">
                            <input
                                type="checkbox"
                                checked={isActive(packetViewModel.id, packetField.index)}
                                onchange={(e) => {
                                    const target = e.target as HTMLInputElement;
                                    setActive(packetViewModel.id, packetField.index, target.checked);
                                }}
                            />
                            {packetField.name}
                        </label>
                    }}</For>
                </>
            )}</For>
        </div>

        <div class="flex items-center justify-center">
            <button
                class=" w-[10%] h-[10%] rounded-5 border-none text-center"
                onClick={() => {
                    setDisplays(displays.filter((graph, index) => index !== props.index));
                    props.closeModal({});
                }}>
                <img alt="Delete" src={closeIcon} class="w-full h-full dark:invert justify-center" draggable={false} />
            </button>
        </div>
    </DefaultModalLayout>;
};

export default ReadoutSettingsModal;