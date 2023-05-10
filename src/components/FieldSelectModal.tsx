import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX } from "solid-js";
import { FieldInPacket } from "./FieldsScreen";
import { useBackend } from "./BackendProvider";
import { PacketComponent, PacketComponentType, PacketField, PacketViewModel } from "../backend_interop/types";

/**
 * The properties required for the {@link FieldSelectModal} component.
 */
export type FieldSelectModalProps = {
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

/**
 * A modal component that allows a user to modify the fields contained in a screen.
 * 
 * @param props an object that contains a function to close the modal, the list of fields that are selected, and a callback to select a field
 */
const FieldSelectModal = (props: ModalProps<FieldSelectModalProps>): JSX.Element => {
    const { packetViewModels } = useBackend();

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={packetViewModels}>
                {(packetViewModel: PacketViewModel) =>
                    <div class='flex flex-col bg-neutral-200 p-2'>
                        <h3 class="m-2">{packetViewModel.name}</h3>
                        <For each={packetViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                            {(packetComponent: PacketComponent) => {
                                const field = packetComponent.data as PacketField;
                                return (
                                    <label>
                                        <input type="checkbox"
                                            // Check this checkbox by default if the field has already been selected
                                            checked={props.selectedFields.some(selectedField => selectedField.packetId === packetViewModel.id && selectedField.fieldIndex === field.index)}
                                            onclick={(event) => props.handleSelect((event.target as HTMLInputElement).checked, packetViewModel.id, field.index)} />
                                        {field.name}
                                    </label>
                                );
                            }}
                        </For>
                    </div>
                }
            </For>
        </DefaultModalLayout>
    );
};

export default FieldSelectModal;