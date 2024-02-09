import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX, createSignal } from "solid-js";
import { FieldInPacket, GraphStruct } from "./FieldsScreen";
import { useBackend } from "./BackendProvider";
import { PacketComponent, PacketComponentType, PacketField, PacketStructureViewModel } from "../backend_interop/types";
import edit from "../assets/edit.png";

/**
 * The properties required for the {@link FieldSelectModal} component.
 */
export type FieldSelectModalProps = {
    // /**
    //  * The currently selected field for x-axis
    //  */
    //xSelectedField: FieldInPacket,
    graph: GraphStruct
    handleSelectY: (isChecked: boolean, fieldIndex: number, index: number) => void
    handleSelectX: (isChecked: boolean, fieldIndex: number, index: number) => void
    index: number
    // /**
    //  * The list of currently selected fields for the y-axis
    //  */
    //ySelectedFields: FieldInPacket[],

    /**
     * A function that toggles the inclusion of the X-Axis field with the given packet id and field index in the current screen.
     * 
     * @param packetId the id of the packet containing the field to toggle the selection of
     * @param fieldIndex the index of the field in the packet to toggle the selection of
     */
    // handleXAxisSelect: (packetId: number, fieldIndex: number) => void
    // /**
    //  * A function that toggles the inclusion of the Y-Axis field with the given packet id and field index in the current screen.
    //  * 
    //  * @param isChecked `true` if the checkbox is checked, `false` otherwise
    //  * @param packetId the id of the packet containing the field to toggle the selection of
    //  * @param fieldIndex the index of the field in the packet to toggle the selection of
    //  */
    // handleYAxisSelect: (isChecked: boolean, packetId: number, fieldIndex: number) => void
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
                {(packetViewModel: PacketStructureViewModel) =>
                    <div class='flex flex-col bg-neutral-200 p-2 rounded-10'>
                        <h3 style="text-align:center;" class="m-2">
                            {props.graph.graphName}
                            {/* <button style = "absolute p-2">
                                <img src={edit} alt="wrong" height={10} draggable={false} />
                            </button> */}
                        </h3>
                        <div class='flex flex-row bg-neutral-200 p-2 rounded-10'>

                            <div class='flex flex-col bg-neutral-200 p-2 rounded-10'>
                                <h2>X-Axis</h2>
                                <For each={packetViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                    {(packetComponent: PacketComponent) => {
                                        const field = packetComponent.data as PacketField;
                                        return (
                                            <label>
                                                <input type="radio"
                                                    checked={props.graph.x === field.index} // Check based on the state
                                                    onclick={(event) => 
                                                        props.handleSelectX((event.target as HTMLInputElement).checked, field.index, props.index)
                                                    }
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
                                                    checked={props.graph.y.some(selectedField => selectedField === field.index)} 
                                                    onclick={(event) => {
                                                        //props.graph.y.some((event.target as HTMLInputElement).checked, 
                                                        //props.graph.y.push(field.index)
                                                        props.handleSelectY((event.target as HTMLInputElement).checked, field.index, props.index)
                                                        
                                                        // packetViewModel.id
                                                        //)
                                                    
                                                    }} />
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