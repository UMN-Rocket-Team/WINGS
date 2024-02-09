import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { For, JSX, createSignal } from "solid-js";
import { FieldInPacket, GraphStruct } from "./FieldsScreen";
import { useBackend } from "./BackendProvider";
import { PacketComponent, PacketComponentType, PacketField, PacketStructureViewModel } from "../backend_interop/types";
import closeIcon from "../assets/close.svg";

/**
 * The properties required for the {@link FieldSelectModal} component.
 * Technically because of recent changes, we can implement all of these functions locally and access the graphStruct
 * but for readibility we will leave it as is for now -Adit
 */
export type FieldSelectModalProps = {
    /** Graph that is being passed */
    graph: GraphStruct
    /* HandleSelectY function to update the y axis of the given graph */
    handleSelectY: (isChecked: boolean, fieldIndex: number, index: number) => void
    /** HandleSelectY function to update the y axis of the given graph */
    handleSelectX: (isChecked: boolean, fieldIndex: number, index: number) => void
    /** Index of graph so that handleSelect[Y/X] can be called correctly! */
    index: number
    /** Function to update name of the given graph */
    setGraphName: (newName: string, index: number) => void
    /** Deletes a graph */
    deleteGraph: (index: number) => void
}

/**
 * A modal component that allows a user to modify the fields contained in a screen.
 * 
 * @param props an object that contains a function to close the modal, the list of fields that are selected, and a callback to select a field
 */
const FieldSelectModal = (props: ModalProps<FieldSelectModalProps>): JSX.Element => {
    const { packetViewModels } = useBackend();

    /** Signal used to help handleInput revert from blank inputs to most recent name */
    const [graphCurrName, setName] = createSignal(props.graph.graphName);

    /** handleInput will handle updating the graphs name and also catches blank inputs and reverts to previous name */
    const handleInput = (event: Event) => {
        const newName = (event.target as HTMLElement).textContent || '';
        if (newName.trim() !== '') {
            props.setGraphName(newName.trim(), props.index);
            setName(newName.trim());
        }  else {
            (event.target as HTMLElement).textContent = graphCurrName();
        }
    };

    /* handleKeyDown helps handle updating the graphName by preventing enters(newlines) */
    const handleKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'Enter') {
          event.preventDefault();
        }
    };

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={packetViewModels}>
                {(packetViewModel: PacketStructureViewModel) =>
                    <div class='flex flex-col bg-neutral-200 p-2 rounded-10'>
                        <h3 contenteditable={true}  style="text-align:center;" class="m-2" onBlur={handleInput} onKeyDown={handleKeyDown}>
                            {graphCurrName()}
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
                        <h3 style="text-align:center;" class="m-2">
                            Settings
                            {/* TODO!!! Allow for changing color of the graph object and  */}
                        </h3>
                        <div class = "relative items-center justify-center" style={"text-align:center;"}>
                            <button 
                                class = " w-[10%] h-[10%] rounded-5 border-none justify-center"
                                onClick={() => {
                                    props.deleteGraph(props.index);
                                    props.closeModal({})
                                }}>
                                <img alt="Delete" src={closeIcon} class="w-full h-full dark:invert justify-center" draggable={false} />
                            </button>
                            
                        </div>
                    </div>                  
                    
                }
            </For>
        </DefaultModalLayout>
    );
};

export default FieldSelectModal;