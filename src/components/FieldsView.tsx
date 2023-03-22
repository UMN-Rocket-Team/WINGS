import {Component, For, JSX} from "solid-js";
import {PacketField, PacketViewModel} from "../backend_interop/types";
import {useModal} from "./ModalProvider";
import ExpandedFieldsModal from "./ExpandedFieldsModal";
import upRightArrow from "../assets/up-right-arrow.png"
import broom from "../assets/broom.png"
import {createStore} from "solid-js/store";
import FieldSelectModal, {FieldSelectModalProps} from "./FieldSelectModal";

export type FieldInPacket = {
    packetViewModel: PacketViewModel
    fieldIndex: number
    globalIndex: number
}

export type FieldsViewState = {
    allFieldsInPackets: FieldInPacket[]
}

export type FieldsViewProps = {
    fieldsViewState: FieldsViewState
};

const FieldsView: Component<FieldsViewProps> = (props: FieldsViewProps): JSX.Element => {
    const {showModal} = useModal();

    const [selected, setSelected] = createStore<FieldInPacket[]>([]);

    const handleSelect = (event: Event) => {
        const globalIndex = parseInt((event.target as HTMLSelectElement).value);
        if ((event.target as HTMLInputElement).checked) {
            setSelected([...selected, props.fieldsViewState.allFieldsInPackets[globalIndex]]);
        } else {
            setSelected(selected.filter((fieldInPacket: FieldInPacket) => fieldInPacket.globalIndex !== globalIndex));
        }
    }

    return (
        <div class="relative bg-red p-2">
            {/*Field Select Button*/}
            <button onClick={() => showModal<FieldSelectModalProps, {}>(FieldSelectModal, {fieldViewState: props.fieldsViewState, selectedFields: selected, handleSelect: handleSelect})}>
                Select Fields
            </button>

            {/*Expand button*/}
            <button class="absolute top-1 right-1 w-5 h-5 p-0"
                    onClick={() => showModal<FieldsViewState, {}>(ExpandedFieldsModal, {allFieldsInPackets: selected})}>
                <img src={upRightArrow} style={{"width": "100%", "height": "100%"}} alt="Expand"></img>
            </button>

            {/*Delete button*/}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                    onClick={() => {setSelected([])}}>
                <img src={broom} style={{"width": "100%", "height": "100%"}} alt="Delete"></img>
            </button>

            {/*Fields*/}
            <div class="absolute flex flex-wrap top-10 bottom-8 left-0 right-0 m-a p-4 items-center justify-center gap-4 overflow-y-scroll" style={{"width": "90%"}}>
                <For each={selected}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="bg-gray p-2">
                            <h3>{fieldInPacket.packetViewModel.name}</h3>
                            <p>{(fieldInPacket.packetViewModel.components[fieldInPacket.fieldIndex].data as PacketField).name}</p>
                        </div>
                    }
                </For>
            </div>
        </div>
    )
}

export default FieldsView;