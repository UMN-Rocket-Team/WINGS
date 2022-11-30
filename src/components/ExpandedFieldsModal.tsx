import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {JSX} from "solid-js";

const ExpandedFieldsModal = (props: ModalProps): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Packet">
            <p>Name:</p>
        </DefaultModalLayout>
    );
};

export default ExpandedFieldsModal;