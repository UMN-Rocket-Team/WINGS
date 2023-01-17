import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import {FieldInPacket, FieldsViewState} from "./FieldsView";
import SolidChart from "./SolidChart";

const ExpandedFieldsModal = (props: ModalProps<FieldsViewState>): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})}>
            <div class="flex flex-row gap-2 bg-red">
                <For each={props.fieldsInPackets}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="p-2">
                            <p>{fieldInPacket.packetStructure.name} {fieldInPacket.packetStructure.fields[fieldInPacket.fieldIndex].name}</p>
                            <SolidChart/>
                        </div>
                    }
                </For>
            </div>
        </DefaultModalLayout>
    );
};

export default ExpandedFieldsModal;