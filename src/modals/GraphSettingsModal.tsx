import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { Accessor, For, JSX, createSignal } from "solid-js";
import { DisplayStruct, SettingsModalProps, displays, setDisplays } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponent, PacketComponentType, PacketField, PacketStructureViewModel } from "../backend_interop/types";
import closeIcon from "../assets/close.svg";
import { produce } from "solid-js/store";

/**
 * generic interface for all g
 */
export interface GraphModalProps extends SettingsModalProps{
    /** Graph that is being passed */
    displayStruct: GraphStruct,
    /** Index of graph so that handleSelect[Y/X] can be called correctly! */
}
export interface GraphStruct extends DisplayStruct{
    x: number, //fieldIndex
    y: number[],
    colors: string[];
}

/**
 * A modal component that allows a user to modify the fields contained in a screen.
 * 
 * @param props an object that contains a function to close the modal, the list of fields that are selected, and a callback to select a field
 */
const FieldSelectModal = (props: ModalProps<GraphModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

    /** Signal used to help handleInput revert from blank inputs to most recent name */
    const [graphCurrName, setName] = createSignal(props.displayStruct.displayName);

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
            setDisplays( produce((s) => {
                if (s[graphIndex].packetID != packet_id){
                    (s[graphIndex] as GraphStruct).y = []
                    s[graphIndex].packetID = packet_id;
                    (s[graphIndex] as GraphStruct).x = 0;//sets x back to 0 to avoid overflow problems
                }
                (s[graphIndex] as GraphStruct).y.push(fieldIndex)
            }));
            // setGraphs( produce((s) => {
            // }));
        } else {
            setDisplays( produce((s) => 
            (s[graphIndex] as GraphStruct).y = (s[graphIndex] as GraphStruct).y.filter(ind => ind != fieldIndex)));
        }
    }
    
    const handleSelectX = (isChecked: boolean, fieldIndex: number, graphIndex: number, packet_id: number) => {
        if (isChecked) {
            setDisplays( produce((s) => {
                if (s[graphIndex].packetID != packet_id){
                    (s[graphIndex] as GraphStruct).y = (s[graphIndex] as GraphStruct).y.filter(_ => false);//sets all y values to false
                    s[graphIndex].packetID = packet_id;
                }
                (s[graphIndex] as GraphStruct).x = fieldIndex
            }));
        } else {
            setDisplays( produce((s) => 
            (s[graphIndex] as GraphStruct).x = 0));
        }
    }

    const setGraphName = (newName: string, index: number) => {
        setDisplays( produce((s) => 
                s[index].displayName = newName))
    }

    const deleteGraph = (index: number) => {
        let newGraphs: DisplayStruct[] = [];
        for (let i = 0; i < displays.length; i++) {
            if (index !== i) {
                newGraphs.push(displays[i]);
            }
        }
        setDisplays(newGraphs);
    }

    const updateColor = (color: string, colorIndex: number, graphIndex: number) => {
        setDisplays( produce((s) => 
            (s[graphIndex] as GraphStruct).colors[colorIndex] = color))
    }
    
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
                    <div class='flex flex-col bg-neutral-200 dark:bg-gray p-2 rounded-10'>
                        <h3 contenteditable={true}  style="text-align:center;" class="m-2" onBlur={handleInput} onKeyDown={handleKeyDown}>
                            {graphCurrName()}
                        </h3>
                        <For each={PacketStructureViewModels}>
                                {(PacketStructureViewModel: PacketStructureViewModel) =>
                        <div class='flex flex-row bg-neutral-200 dark:bg-gray p-2 rounded-10'>

                            <div class='flex flex-col bg-neutral-200 dark:bg-gray p-2 rounded-10'>
                                <h2>X-Axis</h2>
                                <For each={PacketStructureViewModel.components.filter(component => component.type === PacketComponentType.Field)}>
                                    {(packetComponent: PacketComponent) => {
                                        const field = packetComponent.data as PacketField;
                                        return (
                                            <label>
                                                <input type="radio"
                                                    checked={props.displayStruct.x === field.index && props.displayStruct.packetID === PacketStructureViewModel.id} // Check based on the state
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
                                                    checked={props.displayStruct.y.some(selectedField => selectedField === field.index) && props.displayStruct.packetID === PacketStructureViewModel.id} 
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
                        }
                        </For>
                        <h3 style="text-align:center;" class="m-2">
                            Settings
                            {/* TODO!!! Allow for changing color of the graph object and variables */}
                        </h3>
                        
                        {/* Below is the set up to create a color picker for each var, in progress still. */}
                        <div class = "flex flex-col bg-neutral-200 dark:bg-gray p-2" style={"text-align:center;"}>
                            <h2>Graph Colors</h2>
                            <For each={PacketStructureViewModels.find(psViewModel => psViewModel.id === props.displayStruct.packetID)?.components.filter(component => component.type === PacketComponentType.Field)}>
                                {(packetComponent: PacketComponent, i) => {
                                    const field = packetComponent.data as PacketField;
                                    return (
                                        <label>
                                            {field.name}
                                            <input type="color" style={"rounded-full"} value={props.displayStruct.colors[i() % props.displayStruct.colors.length]} onInput={(event) => {
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
        </DefaultModalLayout>
    );
};

export default FieldSelectModal;