import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "../core/DefaultModalLayout";
import {JSX} from "solid-js";

/**
 * A simple modal component that notifies the use that the file save was sucessful.
 * 
 * @param props an object that contains a function to close the modal
 */
const SaveModal = (props: ModalProps): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Save was successful!"></DefaultModalLayout>
    );
};

export default SaveModal;