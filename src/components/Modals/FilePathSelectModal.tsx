import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";

/**
 * The properties required for the {@link ErrorModal} component.
 */
export type FileModalProps = {
    /**
     * list of paths
     */
    pathStrings: string[]
    
    /**
     * call this with the string returned from the modal 
     */
    callBack: Function
};

/**
 * A simple modal component that gives the user a list of directories, and the option to select their own.
 * 
 * @param props an object that contains a function to close the modal and the error message and description
 */
const FileModal = (props: ModalProps<FileModalProps>): JSX.Element => {

    const runCallBack = (filePath: string) => {
        props.callBack(filePath);
        props.closeModal({});
    }

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="File Select">
            <button onClick={() => selectFile}>Select Directory</button>
            <p>Open Recent:</p>
            <For each={props.pathStrings}>{(item) => 
                <div>
                    <button onClick={() => runCallBack(item)}>{item}</button>
                </div>
            }</For>
        </DefaultModalLayout>
    );
};

export default FileModal;