import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import {FieldInPacket, FieldsViewState} from "./FieldsView";
import { PacketField } from "../backend_interop/types";

export type FieldSelectModalProps = {
    fieldViewState: FieldsViewState,
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

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={Object.keys(groupedFields)}>
                {(packetId: string, packetIndex) => {
                    const packetIdNum = parseInt(packetId);
                    return (
                        <div class={packetBackgroundColors[packetIndex() % packetBackgroundColors.length] + " p-2"}>
                            <h3 class="m-2">{groupedFields[packetIdNum][0].packetViewModel.name}</h3>
                            <For each={groupedFields[packetIdNum]}>
                                {(fieldInPacket: FieldInPacket, index) =>
                                    <div>
                                        <input type="checkbox" value={index()} onclick={props.handleSelect}/>
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