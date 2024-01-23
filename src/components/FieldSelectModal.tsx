import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX, createSignal } from "solid-js";
import { FieldInPacket as GraphStruct } from "./FieldsScreen";
import { useBackend } from "./BackendProvider";
import { PacketComponent, PacketComponentType, PacketField, PacketStructureViewModel } from "../backend_interop/types";


/**
 * The properties required for the {@link FieldSelectModal} component.
 */
export type FieldSelectModalProps = {
    // /**
    //  * The currently selected field for x-axis
    //  */
    // xSelectedField: GraphStruct,

    // /**
    //  * The list of currently selected fields for the y-axis
    //  */
    // ySelectedFields: GraphStruct[],

    /**
     * A function that toggles the inclusion of the X-Axis field with the given packet id and field index in the current screen.
     * 
     * @param packetId the id of the packet containing the field to toggle the selection of
     * @param fieldIndex the index of the field in the packet to toggle the selection of
     */
    handleXAxisSelect: (packetId: number, fieldIndex: number) => void
    /**
     * A function that toggles the inclusion of the Y-Axis field with the given packet id and field index in the current screen.
     * 
     * @param isChecked `true` if the checkbox is checked, `false` otherwise
     * @param packetId the id of the packet containing the field to toggle the selection of
     * @param fieldIndex the index of the field in the packet to toggle the selection of
     */
    handleYAxisSelect: (isChecked: boolean, packetId: number, fieldIndex: number) => void
}

const [selectedRadio, setSelectedRadio] = createSignal<number>(-1);

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
                {(packetViewModel: PacketStructureViewModel) =>
                    <div class='flex flex-col bg-neutral-200 p-2'>
                        <h3 style="text-align:center;" class="m-2">{packetViewModel.name}</h3>
                        <div class='flex flex-row bg-neutral-200 p-2'>

                            <div class='flex flex-col bg-neutral-200 p-2'>
                                <h2>X-Axis</h2>
                                <For each={packetViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                    {(packetComponent: PacketComponent) => {
                                        const field = packetComponent.data as PacketField;
                                        return (
                                            <label>
                                                <input type="radio"
                                                    checked={selectedRadio() === field.index} // Check based on the state
                                                    onclick={() => setSelectedRadio(field.index)}
                                                />
                                                {field.name}
                                            </label>
                                        );
                                    }}
                                </For>
                            </div>
                            <div class='flex flex-col bg-neutral-200 p-2'>
                                <h2>Y-Axis</h2>
                                <For each={packetViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                    {(packetComponent: PacketComponent) => {
                                        const field = packetComponent.data as PacketField;
                                        return (
                                            <label>
                                                <input type="checkbox"
                                                    // Check this checkbox by default if the field has already been selected
                                                    checked={props.ySelectedFields.some(selectedField => selectedField.packetId === packetViewModel.id && selectedField.fieldIndex === field.index)} // TODO: This breaks stuff. also X axis needs to be seperated
                                                    onclick={(event) => props.handleYAxisSelect((event.target as HTMLInputElement).checked, packetViewModel.id, field.index)} />
                                                {field.name}
                                            </label>
                                        );
                                    }}
                                </For>
                            </div>
                        </div>  
                    </div>                  
                    
                }
            </For>
        </DefaultModalLayout>
    );
};

export default FieldSelectModal;