import {ModalProps} from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import {JSX} from "solid-js";

const UploadModal = (props: ModalProps): JSX.Element => {
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Upload">
            <div class="flex">
                <p class="pr-2">Name :</p>
                <input name="Name"/>
            </div>
            <div class="flex">
                <p class="pr-2">Public</p>
                <input type="checkbox"/>
            </div>
            <div class="flex place-content-center">
                <button>Upload to Cloud</button>
            </div>
        </DefaultModalLayout>
    );
};

export default UploadModal;