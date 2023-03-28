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
    fieldsInPackets: FieldInPacket[];
    number: number;
}

export type FieldsViewProps = {
    fieldsViewState: FieldsViewState
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
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2">
            {/*Field Select Button*/}
            <button onClick={() => showModal<FieldSelectModalProps, {}>(FieldSelectModal, { fieldViewState: props.fieldsViewState, selectedFields: selected, handleSelect: handleSelect })}>
                Select Fields
            </button>

            {/*Expand button*/}
            <button class="absolute top-1 right-1 w-5 h-5 p-0"
                onClick={() => showModal<FieldsViewState, {}>(ExpandedFieldsModal, { fieldsInPackets: selected, number: props.fieldsViewState.number })}>
                <img src={upRightArrow} style={{ "width": "100%", "height": "100%" }} alt="Expand"></img>
            </button>

            {/*Delete button*/}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                onClick={() => { setSelected([]) }}>
                <img src={broom} style={{ "width": "100%", "height": "100%" }} alt="Delete"></img>
            </button>

            {/*Fields*/}
            <div class="absolute flex flex-wrap top-10 bottom-8 left-0 right-0 m-a p-4 items-center justify-center gap-4 overflow-y-scroll" style={{"width": "90%"}}>
                <For each={selected}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="bg-gray p-2">
                            <h3>{fieldInPacket.packetName}</h3>
                            <p>{fieldInPacket.name}</p>
                        </div>
                    }
                </For>
            </div>
        </div>
    )
}

export default FieldsView;
