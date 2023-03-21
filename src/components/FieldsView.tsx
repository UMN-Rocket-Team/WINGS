import { Component, For, JSX } from "solid-js";
import { useModal } from "./ModalProvider";
import ExpandedFieldsModal from "./ExpandedFieldsModal";
import upRightArrow from "../assets/up-right-arrow.png"
import broom from "../assets/broom.png"
import { createStore } from "solid-js/store";
import FieldSelectModal, { FieldSelectModalProps } from "./FieldSelectModal";

export type FieldInPacket = {
    packetName: string,
    packetId: number,
    name: string,
    fieldIndex: number,
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

    const [selected, setSelected] = createStore<FieldInPacket[]>([]);

    const handleSelect = (event: Event) => {
        const [selectedPacketId, selectedFieldIndex] = JSON.parse((event.target as HTMLSelectElement).value) as number[];
        if ((event.target as HTMLInputElement).checked) {
            setSelected([...selected, props.fieldsViewState.fieldsInPackets.find(
                fieldInPacket => fieldInPacket.packetId === selectedPacketId && fieldInPacket.fieldIndex === selectedFieldIndex)!]);
        } else {
            setSelected(selected.filter(
                fieldInPacket => fieldInPacket.packetId !== selectedPacketId || fieldInPacket.fieldIndex !== selectedFieldIndex));
        }
    }

    return (
        <div class="relative bg-red p-2">
            {/*Field Select Button*/}
            <button onClick={() => showModal<FieldSelectModalProps, {}>(FieldSelectModal, { fieldViewState: props.fieldsViewState, handleSelect: handleSelect })}>
                Select Fields
            </button>

            {/*Expand button*/}
            <button class="absolute top-1 right-1 w-5 h-5 p-0"
                onClick={() => showModal<FieldsViewState, {}>(ExpandedFieldsModal, { fieldsInPackets: selected })}>
                <img src={upRightArrow} style={{ "width": "100%", "height": "100%" }} alt="Expand"></img>
            </button>

            {/*Delete button*/}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                onClick={() => props.deleteFieldsView(props.fieldsViewState)}>
                <img src={broom} style={{ "width": "100%", "height": "100%" }} alt="Delete"></img>
            </button>

            {/*Fields*/}
            <div class="flex items-center justify-center gap-4" style={{ "height": "100%" }}>
                <For each={selected}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="bg-gray p-2" style="height: 80%">
                            <p>{fieldInPacket.packetName}</p>
                            <p>{fieldInPacket.name}</p>
                        </div>
                    }
                </For>
            </div>
        </div>
    )
}

export default FieldsView;
