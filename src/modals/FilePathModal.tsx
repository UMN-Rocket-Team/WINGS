import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {For, JSX} from "solid-js";
import { runImportPacketWindow } from "../core/file_handling";

/**
 * The properties required for the {@link ErrorModal} component.
 */
export type FileModalProps = {
    /**
     * list of paths in reverse order of what it will be displayed (last item will be on top)
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

    const runCallBack = (filePaths: string[] | string | null) => {
        props.callBack(filePaths);
        props.closeModal({});
    }

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="File Select">
            <button class="listButton" onClick={async () => runCallBack(await runImportPacketWindow())}>Select Directory</button>
            <p>Open Recent:</p>
            <For each={props.pathStrings.reverse()}>{(item) => 
                <div>
                    <button class="listButton" onClick={() => runCallBack([item])}>{item}</button>
                </div>
            }</For>
        </DefaultModalLayout>
    );
};

export default FileModal;