import { Component, For } from "solid-js";
import { useModal } from "./ModalProvider";
import ExpandedFieldsModal, { ExpandedFieldsModalProps } from "./ExpandedFieldsModal";
import { createStore } from "solid-js/store";
import FieldSelectModal, { FieldSelectModalProps } from "./FieldSelectModal";
import { useBackend } from "./BackendProvider";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import expandIcon from "../assets/expand.svg";
import closeIcon from "../assets/close.svg";

/**
 * An object that identifies a field in a packet by its packet id and field index and contains the name of the packet and field.
 */
export type FieldInPacket = {
    /**
     * The id of the packet that contains the field this `FieldInPacket` represents.
     */
    packetId: number,
    /**
     * The index of the field that this `FieldInPacket` represents inside the packet that contains the field.
     */
    fieldIndex: number,
}

/**
 * The properties required for the {@link FieldsScreen} component.
 */
export type FieldsScreenProps = {
    /**
     * The user-displayable number of this screen
     */
    number: number;
};

/**
 * A component that:
 * - Fisplays a list of selected fields added to this screen
 * - Allows users to add fields to the screen
 * - Allows users to clear the screen
 * - Allows users to view the graphed data received for the selected fields
 *
 * @param props an object that contains the number of this screen
 */
const FieldsScreen: Component<FieldsScreenProps> = (props) => {
    const { packetViewModels } = useBackend();
    const { showModal } = useModal();

    const [selected, setSelected] = createStore<FieldInPacket[]>([]);

    const handleSelect = (isChecked: boolean, packetId: number, fieldIndex: number) => {
        if (isChecked) {
            setSelected([...selected, { packetId: packetId, fieldIndex: fieldIndex }]);
        } else {
            setSelected(selected.filter(
                fieldInPacket => fieldInPacket.packetId !== packetId || fieldInPacket.fieldIndex !== fieldIndex));
        }
    }

    return (
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2">
            {/*Field Select Button*/}
            <button onClick={() => showModal<FieldSelectModalProps, {}>(FieldSelectModal, {
                selectedFields: selected,
                handleSelect: handleSelect
            })}>
                Select Fields
            </button>

            {/*Expand button*/}
            <button class="absolute top-1 right-1 w-5 h-5 p-0"
                onClick={() => showModal<ExpandedFieldsModalProps, {}>(ExpandedFieldsModal, {
                    selectedFields: selected,
                    number: props.number
                })}>
                <img alt="Expand" src={expandIcon} class="w-full h-full dark:invert" draggable={false} />
            </button>

            {/*Delete button*/}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                onClick={() => {
                    setSelected([])
                }}>
                <img alt="Delete" src={closeIcon} class="w-full h-full dark:invert" draggable={false} />
            </button>

            {/*Fields*/}
            <div
                class="absolute flex flex-wrap top-10 bottom-8 left-0 right-0 m-a p-4 items-center justify-center gap-4 overflow-y-scroll"
                style={{ "width": "90%" }}>
                <For each={selected}>
                    {(fieldInPacket: FieldInPacket) => {
                        const packetViewModel = packetViewModels.find(packetViewModel => packetViewModel.id === fieldInPacket.packetId);
                        const field = packetViewModel?.components.find(component => component.type === PacketComponentType.Field && (component.data as PacketField).index === fieldInPacket.fieldIndex);

                        return (
                            <div class="bg-gray p-2">
                                <h3>{packetViewModel?.name}</h3>
                                <p>{(field?.data as PacketField)?.name}</p>
                            </div>
                        );
                    }}
                </For>
            </div>
        </div>
    )
}

export default FieldsScreen;
