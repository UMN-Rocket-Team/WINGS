import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import {FieldInPacket, FieldsViewState} from "./FieldsView";

export type FieldSelectModalProps = {
    fieldViewState: FieldsViewState,
    handleSelect: (event: Event) => void
}

const FieldSelectModal = (props: ModalProps<FieldSelectModalProps>): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={props.fieldViewState.fieldsInPackets}>
                {(fieldInPacket: FieldInPacket) =>
                    <div>
                        <input type="checkbox" value={JSON.stringify([fieldInPacket.packetId, fieldInPacket.fieldIndex])} onclick={props.handleSelect}/>
                        <label>{fieldInPacket.packetName + ": " + fieldInPacket.name}</label>
                    </div>
                }
            </For>
        </DefaultModalLayout>
    );
};

export default FieldSelectModal;