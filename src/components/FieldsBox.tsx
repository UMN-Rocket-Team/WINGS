import {Component, For, JSX} from "solid-js";
import {PacketStructure} from "../backend_interop/types";

export type FieldInPacket = {
    packetStructure: PacketStructure
    fieldInd: number
}

export type FieldsBoxProps = {
    fieldsInPackets: FieldInPacket[]
};

const FieldsBox: Component<FieldsBoxProps> = (props: FieldsBoxProps): JSX.Element => {
    return (
        <div class="flex bg-red p-2 gap-4">
            <For each={props.fieldsInPackets}>
                {(fieldInPacket: FieldInPacket) =>
                    <div class="bg-gray p-2">
                        <p>{fieldInPacket.packetStructure.name}</p>
                        <p>{fieldInPacket.packetStructure.fields[fieldInPacket.fieldInd].name}</p>
                    </div>
                }
            </For>

            <button>Expand Fields</button>
        </div>
    )
}

export default FieldsBox;