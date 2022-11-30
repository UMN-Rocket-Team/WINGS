import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {JSX} from "solid-js";

const BroadcastModal = (broadcastProps: ModalProps): JSX.Element => {
    return (
        <DefaultModalLayout close={() => broadcastProps.closeModal({})} title="Broadcast">
            <p>Name:</p>
            <input name="Name"/>
        </DefaultModalLayout>
    );
};

export default BroadcastModal;