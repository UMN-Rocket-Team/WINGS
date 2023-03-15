import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import {FieldInPacket, FieldsViewState} from "./FieldsView";
import SolidChart from "./SolidChart";
import { PacketField } from "../backend_interop/types";

const ExpandedFieldsModal = (props: ModalProps<FieldsViewState>): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})}>
            <div class="flex flex-row gap-2 bg-red">
                {/*TODO: update "props"'s field name to be "fieldsInPackets" rather than "allFieldsInPackets"*/}
                <For each={props.allFieldsInPackets}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="p-2">
                            <p>{fieldInPacket.packetViewModel.name} {(fieldInPacket.packetViewModel.components[fieldInPacket.fieldIndex].data as PacketField).name}</p>
                            <SolidChart/>
                        </div>
                    }
                </For>
            </div>
        </DefaultModalLayout>
    );
};

export default ExpandedFieldsModal;