import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import {FieldInPacket, FieldsBoxProps} from "./FieldsBox";

const ExpandedFieldsModal = (props: ModalProps<FieldsBoxProps>): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})}>
            <div class="flex flex-row gap-2">
                <For each={props.fieldsInPackets}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="bg-red p-2">
                            <p>{fieldInPacket.packetStructure.name} {fieldInPacket.packetStructure.fields[fieldInPacket.fieldIndex].name}</p>
                            <h2>[Insert graph here]</h2>
                        </div>
                    }
                </For>
            </div>
        </DefaultModalLayout>
    );
};

export default ExpandedFieldsModal;