import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { Accessor, For, JSX, createSignal } from "solid-js";
import { graphs, GraphStruct, setGraphs } from "../components/FieldsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponent, PacketComponentType, PacketField, PacketStructureViewModel } from "../backend_interop/types";
import closeIcon from "../assets/close.svg";
import { produce } from "solid-js/store";

/**
 * The properties required for the {@link FieldSelectModal} component.
 * Technically because of recent changes, we can implement all of these functions locally and access the graphStruct
 * but for readability we will leave it as is for now -Adit
 */
export type FieldSelectModalProps = {
    /** Graph that is being passed */
    graph: GraphStruct
    /** Index of graph so that handleSelect[Y/X] can be called correctly! */
    index: number
}

/**
 * A modal component that allows a user to modify the fields contained in a screen.
 * 
 * @param props an object that contains a function to close the modal, the list of fields that are selected, and a callback to select a field
 */
const FieldSelectModal = (props: ModalProps<FieldSelectModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

    /** Signal used to help handleInput revert from blank inputs to most recent name */
    const [graphCurrName, setName] = createSignal(props.graph.graphName);

    /** handleInput will handle updating the graphs name and also catches blank inputs and reverts to previous name */
    const handleInput = (event: Event) => {
        const newName = (event.target as HTMLElement).textContent || '';
        if (newName.trim() !== '') {
            setGraphName(newName.trim(), props.index);
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

    const handleSelectY = (isChecked: boolean, fieldIndex: number, graphIndex: number, packet_id: number) => {
        if (isChecked) {
            setGraphs( produce((s) => {
                if (s[graphIndex].packetID != packet_id){
                    s[graphIndex].y = []
                    s[graphIndex].packetID = packet_id;
                    s[graphIndex].x = 0;//sets x back to 0 to avoid overflow problems
                }
                s[graphIndex].y.push(fieldIndex)
            }));
            // setGraphs( produce((s) => {
            // }));
        } else {
            setGraphs( produce((s) => 
                s[graphIndex].y = s[graphIndex].y.filter(ind => ind != fieldIndex)));
        }
    }
    
    const handleSelectX = (isChecked: boolean, fieldIndex: number, graphIndex: number, packet_id: number) => {
        if (isChecked) {
            setGraphs( produce((s) => {
                if (s[graphIndex].packetID != packet_id){
                    s[graphIndex].y = s[graphIndex].y.filter(_ => false);//sets all y values to false
                    s[graphIndex].packetID = packet_id;
                }
                s[graphIndex].x = fieldIndex
            }));
        } else {
            setGraphs( produce((s) => 
                s[graphIndex].x = 0));
        }
    }

    const setGraphName = (newName: string, index: number) => {
        setGraphs( produce((s) => 
                s[index].graphName = newName))
    }

    const deleteGraph = (index: number) => {
        let newGraphs: GraphStruct[] = [];
        for (let i = 0; i < graphs.length; i++) {
            if (index !== i) {
                newGraphs.push(graphs[i]);
            }
        }
        setGraphs(newGraphs);
    }

    const updateColor = (color: string, colorIndex: number, graphIndex: number) => {
        setGraphs( produce((s) => 
            s[graphIndex].colors[colorIndex] = color))
    }
    
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={PacketStructureViewModels}>
                {(PacketStructureViewModel: PacketStructureViewModel) =>
                    <div class='flex flex-col bg-neutral-200 dark:bg-gray p-2 rounded-10'>
                        <h3 contenteditable={true}  style="text-align:center;" class="m-2" onBlur={handleInput} onKeyDown={handleKeyDown}>
                            {graphCurrName()}
                        </h3>
                        <div class='flex flex-row bg-neutral-200 dark:bg-gray p-2 rounded-10'>

                            <div class='flex flex-col bg-neutral-200 dark:bg-gray p-2 rounded-10'>
                                <h2>X-Axis</h2>
                                <For each={PacketStructureViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                    {(packetComponent: PacketComponent) => {
                                        const field = packetComponent.data as PacketField;
                                        return (
                                            <label>
                                                <input type="radio"
                                                    checked={props.graph.x === field.index && props.graph.packetID === PacketStructureViewModel.id} // Check based on the state
                                                    onclick={(event) => 
                                                        handleSelectX((event.target as HTMLInputElement).checked, field.index, props.index, PacketStructureViewModel.id)
                                                    }
                                                />
                                                {field.name}
                                            </label>
                                        );
                                    }}
                                </For>
                            </div>
                            <div class='flex flex-col bg-neutral-200 dark:bg-gray p-2'>
                                <h2>Y-Axis</h2>
                                <For each={PacketStructureViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                    {(packetComponent: PacketComponent) => {
                                        const field = packetComponent.data as PacketField;
                                        return (
                                            <label>
                                                <input type="checkbox"
                                                    checked={props.graph.y.some(selectedField => selectedField === field.index) && props.graph.packetID === PacketStructureViewModel.id} 
                                                    onclick={(event) => {
                                                        handleSelectY((event.target as HTMLInputElement).checked, field.index, props.index, PacketStructureViewModel.id);
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
                            {/* TODO!!! Allow for changing color of the graph object and variables */}
                        </h3>
                        
                        {/* Below is the set up to create a color picker for each var, in progress still. */}
                        <div class = "flex flex-col bg-neutral-200 dark:bg-gray p-2" style={"text-align:center;"}>
                            <h2>Graph Colors</h2>
                            <For each={PacketStructureViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                {(packetComponent: PacketComponent, i) => {
                                    const field = packetComponent.data as PacketField;
                                    return (
                                        <label>
                                            {field.name}
                                            <input type="color" style={"rounded-full"} value={props.graph.colors[i() % props.graph.colors.length]} onInput={(event) => {
                                                updateColor((event.target as HTMLInputElement).value, i(), props.index);
                                            }}/>
                                        </label>
                                    );
                                }}
                            </For>
                        </div>
                        <div class = "relative items-center justify-center" style={"text-align:center;"}>
                            <button 
                                class = " w-[10%] h-[10%] rounded-5 border-none justify-center"
                                onClick={() => {
                                    deleteGraph(props.index);
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