import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX } from "solid-js";
import { FieldInPacket, FieldsScreenState } from "./FieldsScreen";

/**
 * The properties required for the {@link FieldSelectModal} component.
 */
export type FieldSelectModalProps = {
    /**
     * An object containing a list of fields that can be selected
     */
    fieldViewState: FieldsScreenState,
    /**
     * The list of currently selected fields
     */
    selectedFields: FieldInPacket[],
    /**
     * A function that toggles the inclusion of the field with the given packet id and field index in the current screen.
     * 
     * @param isChecked `true` if the checkbox is checked, `false` otherwise
     * @param packetId the id of the packet containing the field to toggle the selection of
     * @param fieldIndex the index of the field in the packet to toggle the selection of
     */
    handleSelect: (isChecked: boolean, packetId: number, fieldIndex: number) => void
}

const packetBackgroundColors = ["bg-neutral-200", "bg-neutral-400"];

/**
 * A modal component that allows a user to modify the fields contained in a screen.
 * 
 * @param props an object that contains a function to close the modal, the list of fields that can be selected, the list of fields that are selected, and a callback to select a field
 */
const FieldSelectModal = (props: ModalProps<FieldSelectModalProps>): JSX.Element => {
    // Group the given fields by the id of the packet that contains them
    const groupedFields = props.fieldViewState.fieldsInPackets.reduce((groupedFields: Record<number, FieldInPacket[]>, fieldInPacket) => {
        if (groupedFields[fieldInPacket.packetId] === undefined) {
            groupedFields[fieldInPacket.packetId] = [];
        }
        groupedFields[fieldInPacket.packetId].push(fieldInPacket);
        return groupedFields;
    }, {});

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={Object.keys(groupedFields)}>
                {(packetIdString: string, packetIndex) => {
                    const packetId = parseInt(packetIdString);
                    return (
                        // Rotate through the list of background colors
                        <div class={packetBackgroundColors[packetIndex() % packetBackgroundColors.length] + " p-2"}>
                            <h3 class="m-2">{groupedFields[packetId][0].packetName}</h3>
                            <For each={groupedFields[packetId]}>
                                {(fieldInPacket: FieldInPacket) =>
                                    <div>
                                        <input type="checkbox"
                                            // Check this checkbox by default if the field has already been selected
                                            checked={props.selectedFields.some(selectedField => selectedField.packetId === fieldInPacket.packetId && selectedField.fieldIndex === fieldInPacket.fieldIndex)}
                                            onclick={(event) => props.handleSelect((event.target as HTMLInputElement).checked, fieldInPacket.packetId, fieldInPacket.fieldIndex)} />
                                        <label>{fieldInPacket.name}</label>
                                    </div>
                                }
                            </For>
                        </div>
                    )
                }}
            </For>
        </DefaultModalLayout>
    );
};

export default FieldSelectModal;