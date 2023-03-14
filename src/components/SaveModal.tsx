import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {JSX} from "solid-js";

const SaveModal = (props: ModalProps): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Save was successful!"></DefaultModalLayout>
    );
};

export default SaveModal;