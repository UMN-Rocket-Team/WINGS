import { Component, For } from "solid-js";
import { useModal } from "./ModalProvider";
import { createStore, produce } from "solid-js/store";
import FieldSelectModal, { FieldSelectModalProps } from "./FieldSelectModal";
import { useBackend } from "./BackendProvider";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import closeIcon from "../assets/close.svg";


/**
 * An object that identifies a field in a packet by its packet id and field index and contains the name of the packet and field.
 */


export type FieldInPacket = {
    packetId: number,
    fieldIndex: number,
}

export type GraphStruct = {
    graphName: string,
    x: number, //fieldIndex
    y: number[],
}

export const [graphs, setGraph] = createStore<GraphStruct[]>([]);

/**
 * A component that:
 * - Fisplays a list of selected fields added to this screen
 * - Allows users to add fields to the screen
 * - Allows users to clear the screen
 * - Allows users to view the graphed data received for the selected fields
 *
 * @param props an object that contains the number of this screen
 */
const FieldsScreen: Component = () => {
    const { packetViewModels } = useBackend();
    const { showModal } = useModal();

    const handleSelectY = (isChecked: boolean, fieldIndex: number, index: number) => {
        if (isChecked) {
            setGraph( produce((s) => {
                s[index].y.push(fieldIndex)}))
        } else {
            setGraph( produce((s) => 
                s[index].y = s[index].y.filter(ind => ind != fieldIndex)));
        }
    }
    const handleSelectX = (isChecked: boolean, fieldIndex: number, index: number) => {
        if (isChecked) {
            setGraph( produce((s) => 
                s[index].x = fieldIndex));
        } else {
            setGraph( produce((s) => 
                s[index].x = 0));
        }
    }

    const setGraphName = (newName: string, index: number) => {
        setGraph( produce((s) => 
                s[index].graphName = newName))
    }

    const deleteGraph = (index: number) => {
        let newGraphs: GraphStruct[] = [];
        for (let i = 0; i < graphs.length; i++) {
            if (index !== i) {
                newGraphs.push(graphs[i]);
            }
        }
        setGraph(newGraphs);
    }

    let counter = 1;
    return (
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2">
            {/*Field Select Button*/}
            <button onClick={() => 
            {setGraph([...graphs, {graphName: `Graph ${counter}`, x: 0, y: [0]}]);
                {counter = counter + 1};
            }}>
                New Graph
            </button>
            
            {/* Delete button  */}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                onClick={() => {
                    setGraph([])
                    counter = 1
                }}>
                <img alt="Delete" src={closeIcon} class="w-full h-full dark:invert" draggable={false} />
            </button>

            {/*Fields*/}
            <div
                class="absolute flex flex-wrap top-10 bottom-8 left-0 right-0 m-a p-4 items-center justify-center gap-4 overflow-y-scroll"
                style={{ "width": "90%" }}>
                <For each={graphs}>
                    {(graph: GraphStruct, index) => {
                        const packetViewModel = packetViewModels.find(packetViewModel => packetViewModel.id === graph.x);

                        //Absolutely no clue why this is needed or what it does, but if it is used in a <p> tag inside of the button below and if I remove that the code only displays 1 graph.
                        //Again, no clue why, but it isn't breaking anything as it is right now so This can be something we come back to in the future.
                        const field = packetViewModel?.components.find(component => component.type === PacketComponentType.Field && (component.data as PacketField).index === graph.fieldIndex);

                        return (
                            <div class="bg-stone-400 dark:bg-dark-900 flex justify-center items-center w-[100px] h-[100px] p-1.5 overflow-hidden rounded-7 ">
                                <button 
                                    class = "bg-white w-[100%] h-[100%] rounded-5.5 border-none justify-center dark:bg-dark-300"
                                    onClick={() => showModal<FieldSelectModalProps, {}>(FieldSelectModal, {
                                        graph,
                                        handleSelectY,
                                        handleSelectX,
                                        index:index(),
                                        setGraphName,
                                        deleteGraph
                                    })
                                }>
                                    <h3 class="text-black dark:text-white">{graph.graphName}</h3>
                                    <p>{(field?.data as PacketField)?.name}</p>
                                </button>
                            </div>
                        );
                    }}
                </For>
            </div>
        </div>
    )
}

export default FieldsScreen;
