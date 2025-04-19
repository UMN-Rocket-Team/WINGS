import { ModalProps } from "../core/ModalProvider";
import DefaultModalLayout from "../core/DefaultModalLayout";
import { JSX } from "solid-js";
import { SettingsModalProps } from "../components/DisplaySettingsScreen";
import { DisplayStruct } from "../core/display_registry";

export class RocketStruct implements DisplayStruct {
    displayName = 'Rocket';
    packetID = -1;
    type = 'rocket';
    packetsDisplayed: boolean[] = [false];
}

const RocketSettingsModal = (props: ModalProps<SettingsModalProps>): JSX.Element => {
    return <DefaultModalLayout close={() => props.closeModal({})}>
        <div class="flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg relative min-w-fit">
            <p>This page isn't done yet :(</p>
        </div>
    </DefaultModalLayout>;
};

export default RocketSettingsModal;