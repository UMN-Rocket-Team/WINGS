import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import {FieldInPacket, FieldsViewState} from "./FieldsView";
import { PacketField } from "../backend_interop/types";

export type FieldSelectModalProps = {
    fieldViewState: FieldsViewState,
    handleSelect: (event: Event) => void
}

const FieldSelectModal = (props: ModalProps<FieldSelectModalProps>): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
            <For each={props.fieldViewState.allFieldsInPackets}>
                {(fieldInPacket: FieldInPacket, index) =>
                    <div>
                        <input type="checkbox" value={index()} onclick={props.handleSelect}/>
                        <label>{fieldInPacket.packetViewModel.name + ": " + (fieldInPacket.packetViewModel.components[fieldInPacket.fieldIndex].data as PacketField).name}</label>
                    </div>
                }
            </For>
        </DefaultModalLayout>
    );
};

export default FieldSelectModal;