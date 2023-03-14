import {Component, For, JSX} from "solid-js";
import {PacketField, PacketViewModel} from "../backend_interop/types";
import {useModal} from "./ModalProvider";
import ExpandedFieldsModal from "./ExpandedFieldsModal";
import upRightArrow from "../assets/up-right-arrow.png"
import broom from "../assets/broom.png"
import {createStore} from "solid-js/store";

export type FieldInPacket = {
    packetViewModel: PacketViewModel
    fieldIndex: number
}

export type FieldsViewState = {
    allFieldsInPackets: FieldInPacket[]
}

export type FieldsViewProps = {
    fieldsViewState: FieldsViewState
    deleteFieldsView: (fieldsViewToDelete: FieldsViewState) => void
};

const FieldsView: Component<FieldsViewProps> = (props: FieldsViewProps): JSX.Element => {
    const {showModal} = useModal();

    const [selected, setSelected] = createStore<FieldInPacket[]>([]);

    const handleSelect = (event: Event) => {
        const index = parseInt((event.target as HTMLSelectElement).value);
        setSelected([...selected, props.fieldsViewState.allFieldsInPackets[index]]);
    }

    return (
        <div class="relative bg-red p-2">
            {/*Dropdown list for adding fields*/}
            <select class="absolute top-1 left-1 p-0" name="Add Field" onChange={handleSelect}>
                {props.fieldsViewState.allFieldsInPackets.map((fieldInPacket: FieldInPacket, index: number) => (
                    <option value={index}>
                        {fieldInPacket.packetViewModel.name + ": " + (fieldInPacket.packetViewModel.components[fieldInPacket.fieldIndex].data as PacketField).name}
                    </option>
                ))}
            </select>

            {/*Expand button*/}
            <button class="absolute top-1 right-1 w-5 h-5 p-0"
                    onClick={() => showModal<FieldsViewState, {}>(ExpandedFieldsModal, {allFieldsInPackets: selected})}>
                <img src={upRightArrow} style={{"width": "100%", "height": "100%"}} alt="Expand"></img>
            </button>

            {/*Delete button*/}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                    onClick={() => props.deleteFieldsView(props.fieldsViewState)}>
                <img src={broom} style={{"width": "100%", "height": "100%"}} alt="Delete"></img>
            </button>

            {/*Fields*/}
            <div class="flex items-center justify-center gap-4" style={{"height": "100%"}}>
                <For each={selected}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="bg-gray p-2" style="height: 80%">
                            <p>{fieldInPacket.packetViewModel.name}</p>
                            <p>{(fieldInPacket.packetViewModel.components[fieldInPacket.fieldIndex].data as PacketField).name}</p>
                        </div>
                    }
                </For>
            </div>
        </div>
    )
}

export default FieldsView;