import { Component, For } from "solid-js";
import { useModal } from "../modals/ModalProvider";
import { createStore, produce } from "solid-js/store";
import FieldSelectModal, { FieldSelectModalProps } from "../modals/FieldSelectModal";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import closeIcon from "../assets/close.svg";


/**
 * An object that identifies a field in a packet by its packet id and field index and contains the name of the packet and field.
 */

export type GraphStruct = {
    graphName: string,
    packetID: number,
    x: number, //fieldIndex
    y: number[],
    colors: string[];
}

export const [graphs, setGraphs] = createStore<GraphStruct[]>([]);

/**
 * A component that:
 * - Displays a list of selected fields added to this screen
 * - Allows users to add fields to the screen
 * - Allows users to clear the screen
 * - Allows users to view the graphed data received for the selected fields
 *
 * @param props an object that contains the number of this screen
 */
const FieldsScreen: Component = () => {
    const { PacketStructureViewModels } = useBackend();
    const { showModal } = useModal();

    let counter = 1;
    return (
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2">
            {/*Field Select Button*/}
            <button onClick={() => 
            {   
                if (PacketStructureViewModels.length != 0){
                    setGraphs([...graphs, {graphName: `Graph ${counter}`, packetID: PacketStructureViewModels[0].id, x: 0, y: [0], colors: ["#FFD700", "#0000FF", "#000000", "#FF0000", "#00FF00"]}]);
                    {counter = counter + 1};
                }
            }}>
                New Graph
            </button>
            
            {/* Delete button  */}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                onClick={() => {
                    setGraphs([])
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
                        return (
                            <div class="bg-stone-400 dark:bg-dark-900 flex justify-center items-center w-[100px] h-[100px] p-1.5 overflow-hidden rounded-7 ">
                                <button 
                                    class = "bg-white w-[100%] h-[100%] rounded-5.5 border-none justify-center dark:bg-dark-300"
                                    onClick={() => showModal<FieldSelectModalProps, {}>(FieldSelectModal, {
                                        graph,
                                        index:index(),
                                    })
                                }>
                                    <h3 class="text-black dark:text-white">{graph.graphName}</h3>
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
