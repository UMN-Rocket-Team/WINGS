import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { JSX } from "solid-js";

/**
 * The properties required for the {@link ErrorModal} component.
 */
export type ErrorModalProps = {
    /**
     * The name of the error that occurred, displayed as the title of the modal
     */
    error: string,
    /**
     * A description of the error, displayed as the content of the modal
     */
    description: string;
};

/**
 * A simple modal component that notifies the user that an error has occurred.
 * 
 * @param props an object that contains a function to close the modal and the error message and description
 */
const ErrorModal = (props: ModalProps<ErrorModalProps>): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title={props.error}>
            <p>{props.description}</p>
            <button 
                class="mt-4 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 dark:bg-blue-700 dark:hover:bg-blue-800" 
                onClick={() => props.closeModal({})}>
                Ok
            </button>
        </DefaultModalLayout>
    );
};

export default ErrorModal;
