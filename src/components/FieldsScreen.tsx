import { Component, For, createSignal } from "solid-js";
import { useModal } from "./ModalProvider";
import ExpandedFieldsModal, { ExpandedFieldsModalProps } from "./ExpandedFieldsModal";
import { createStore, produce } from "solid-js/store";
import FieldSelectModal, { FieldSelectModalProps } from "./FieldSelectModal";
import { useBackend } from "./BackendProvider";
import { PacketComponentType, PacketData, PacketField } from "../backend_interop/types";
import expandIcon from "../assets/expand.svg";
import closeIcon from "../assets/close.svg";
import { SolidChartProps } from "./SolidChart";
import { time } from "console";


/**
 * An object that identifies a field in a packet by its packet id and field index and contains the name of the packet and field.
 */


export type FieldInPacket = {
    packetId: number,
    fieldIndex: number,
}

export type GraphStruct = {
    graphName: String,
    x: number, //fieldIndex
    y: number[],
}


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

    const [graphs, setGraph] = createStore<GraphStruct[]>([]);



    const [selected, setSelected] = createStore<FieldInPacket[]>([]);
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
    return (
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2">
            {/*Field Select Button*/}
            <button onClick={() => setGraph([...graphs, {graphName: "Graph", x: 0, y: []}])}>
                New Graph
            </button>

            {/*Expand button*/}
            {/* <button class="absolute top-1 right-1 w-5 h-5 p-0"
                onClick={() => showModal<ExpandedFieldsModalProps, {}>(ExpandedFieldsModal, {
                    selectedFields: selected,
                    number: props.number
                })}>
                <img alt="Expand" src={expandIcon} class="w-full h-full dark:invert" draggable={false} />
            </button> */}

            {/*Delete button*/}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                onClick={() => {
                    setGraph([])
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
                        const field = packetViewModel?.components.find(component => component.type === PacketComponentType.Field && (component.data as PacketField).index === graph.fieldIndex);

                        return (
                            <div class="bg-gray p-1">
                                <button onClick={() => showModal<FieldSelectModalProps, {}>(FieldSelectModal, {
                                    graph,
                                    handleSelectY,
                                    handleSelectX,
                                    index:index()
                                })}>
                                    <h3>{packetViewModel?.name}</h3>
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

export const getGraphs = () => {
    // return graphs;
    return
}