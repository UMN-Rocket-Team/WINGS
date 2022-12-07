import {Component, For, JSX} from "solid-js";
import {PacketStructure} from "../backend_interop/types";
import {useModal} from "./ModalProvider";
import ExpandedFieldsModal from "./ExpandedFieldsModal";

export type FieldInPacket = {
    packetStructure: PacketStructure
    fieldIndex: number
}

export type FieldsBoxProps = {
    fieldsInPackets: FieldInPacket[]
};

const FieldsBox: Component<FieldsBoxProps> = (props: FieldsBoxProps): JSX.Element => {
    const { showModal } = useModal();

    return (
        <div class="flex bg-red p-2 gap-4">
            <For each={props.fieldsInPackets}>
                {(fieldInPacket: FieldInPacket) =>
                    <div class="bg-gray p-2">
                        <p>{fieldInPacket.packetStructure.name}</p>
                        <p>{fieldInPacket.packetStructure.fields[fieldInPacket.fieldIndex].name}</p>
                    </div>
                }
            </For>

            <button onClick={() => showModal<FieldsBoxProps, {}>(ExpandedFieldsModal, props)}>Expand Fields</button>
        </div>
    )
}

export default FieldsBox;