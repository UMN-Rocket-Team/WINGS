import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import {FieldInPacket, FieldsViewState} from "./FieldsView";

export type FieldSelectModalProps = {
    fieldViewState: FieldsViewState,
    handleSelect: (event: Event) => void
}

const packetBackgroundColors = ["bg-red-2", "bg-gray-2"];

const FieldSelectModal = (props: ModalProps<FieldSelectModalProps>): JSX.Element => {
    const groupedFields = props.fieldViewState.fieldsInPackets.reduce((acc: Record<number, FieldInPacket[]>, fieldInPacket) => {
        if (acc[fieldInPacket.packetId] === undefined) {
            acc[fieldInPacket.packetId] = [];
        }
        acc[fieldInPacket.packetId].push(fieldInPacket);
        return acc;
    }, {});

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={Object.keys(groupedFields)}>
                {(packetId: string, packetIndex) => {
                    const packetIdNum = parseInt(packetId);
                    return (
                        <div class={packetBackgroundColors[packetIndex() % packetBackgroundColors.length] + " p-2"}>
                            <h3 class="m-2">{groupedFields[packetIdNum][0].packetName}</h3>
                            <For each={groupedFields[packetIdNum]}>
                                {(fieldInPacket: FieldInPacket) =>
                                    <div>
                                        <input type="checkbox" value={JSON.stringify([fieldInPacket.packetId, fieldInPacket.fieldIndex])} onclick={props.handleSelect}/>
                                        <label>{fieldInPacket.name}</label>
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