import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import {FieldInPacket, FieldsViewState} from "./FieldsView";
import { PacketField } from "../backend_interop/types";

export type FieldSelectModalProps = {
    fieldViewState: FieldsViewState,
    selectedFields: FieldInPacket[],
    handleSelect: (event: Event) => void
}

const packetBackgroundColors = ["bg-red-2", "bg-gray-2"];

const FieldSelectModal = (props: ModalProps<FieldSelectModalProps>): JSX.Element => {
    const groupedFields = props.fieldViewState.allFieldsInPackets.reduce((acc: Record<number, FieldInPacket[]>, fieldInPacket) => {
        if (acc[fieldInPacket.packetViewModel.id] === undefined) {
            acc[fieldInPacket.packetViewModel.id] = [];
        }
        acc[fieldInPacket.packetViewModel.id].push(fieldInPacket);
        return acc;
    }, {});

    const selectedFieldIndices: Set<number> = new Set(props.selectedFields.map(fieldInPacket => fieldInPacket.globalIndex));

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={Object.keys(groupedFields)}>
                {(packetId: string, packetIndex) => {
                    const packetIdNum = parseInt(packetId);
                    return (
                        <div class={packetBackgroundColors[packetIndex() % packetBackgroundColors.length] + " p-2"}>
                            <h3 class="m-2">{groupedFields[packetIdNum][0].packetViewModel.name}</h3>
                            <For each={groupedFields[packetIdNum]}>
                                {(fieldInPacket: FieldInPacket) =>
                                    <div>
                                        <input type="checkbox" value={fieldInPacket.globalIndex} onclick={props.handleSelect} checked={selectedFieldIndices.has(fieldInPacket.globalIndex)}/>
                                        <label>{(fieldInPacket.packetViewModel.components[fieldInPacket.fieldIndex].data as PacketField).name}</label>
                                    </div>
                                }
                            </For>
                        </div>
                    )
                }}
            </For>
        </DefaultModalLayout>
    );
};

export default FieldSelectModal;