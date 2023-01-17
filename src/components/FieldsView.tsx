import {Component, For, JSX} from "solid-js";
import {PacketStructure} from "../backend_interop/types";
import {useModal} from "./ModalProvider";
import ExpandedFieldsModal from "./ExpandedFieldsModal";
import upRightArrow from "../assets/up-right-arrow.png"
import broom from "../assets/broom.png"

export type FieldInPacket = {
    packetStructure: PacketStructure
    fieldIndex: number
}

export type FieldsViewState = {
    fieldsInPackets: FieldInPacket[]
}

export type FieldsViewProps = {
    fieldsViewState: FieldsViewState
    deleteFieldsView: (fieldsViewToDelete: FieldsViewState) => void
};

const FieldsView: Component<FieldsViewProps> = (props: FieldsViewProps): JSX.Element => {
    const { showModal } = useModal();

    return (
        <div class="relative bg-red p-2">
            {/*Fields*/}
            <div class="flex items-center justify-center gap-4" style={{"height": "100%"}}>
                <For each={props.fieldsViewState.fieldsInPackets}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="bg-gray p-2" style="height: 80%">
                            <p>{fieldInPacket.packetStructure.name}</p>
                            <p>{fieldInPacket.packetStructure.fields[fieldInPacket.fieldIndex].name}</p>
                        </div>
                    }
                </For>
            </div>

            {/*Expand button*/}
            <button class="absolute top-1 right-1 w-5 h-5 p-0" onClick={() => showModal<FieldsViewState, {}>(ExpandedFieldsModal, props.fieldsViewState)}>
                <img src={upRightArrow} style={{"width": "100%", "height": "100%"}} alt="Expand"></img>
            </button>

            {/*Delete button*/}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0" onClick={() => props.deleteFieldsView(props.fieldsViewState)}>
                <img src={broom} style={{"width": "100%", "height": "100%"}} alt="Delete"></img>
            </button>
        </div>
    )
}

export default FieldsView;