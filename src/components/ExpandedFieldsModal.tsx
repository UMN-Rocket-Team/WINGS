import { ModalProps } from "./ModalProvider";
import { For, JSX, Show } from "solid-js";
import { GraphStruct } from "./FieldsScreen";
import SolidChart from "./SolidChart";
import closeIcon from "../assets/close.svg";

/**
 * The properties required for the {@link ExpandedFieldsModal} component.
 */
export type ExpandedFieldsModalProps = {
    /**
     * The list of selected packets on this screen
     */
    selectedFields: GraphStruct[];
    /**
     * The user-displayable number of this screen
     */
    number: number;
};

/**
 * A modal component that displays the data received for the given list of fields in graphs. The modal will close when the `Escape` key
 * is pressed or the close button is clicked.
 * 
 * @param props an object that contains a function to close the modal, the list of fields to be displayed, and the number of this screen
 */
const ExpandedFieldsModal = (props: ModalProps<ExpandedFieldsModalProps>): JSX.Element => {
    return (
        <div class="absolute z-10 top-0 left-0 bottom-0 right-0 flex flex-col bg-white dark:bg-dark-700 p-4" tabIndex={-1}
            // Focus the root div of the modal when it is made visible so that it receives keyboard events.
            // The root div of the modal needs to receive keyboard events so that it can close when the Escape key is pressed
            ref={rootElement => setTimeout(() => rootElement.focus())} // Not sure why the setTimeout is necessary, but it is
            onKeyDown={event => {
                // Close the modal if the Escape key is pressed
                if ((event.key || event.code) === "Escape") {
                    props.closeModal({});
                }
            }}>
            <button class="absolute w-8 h-8 right-4 top-4 p-1 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-900 border-rounded aspect-square"
                onClick={() => props.closeModal({})}>
                <img src={closeIcon} class="w-full h-full dark:invert" draggable={false} />
            </button>
            <b class="text-center text-4xl dark:text-white">{`Screen ${props.number}`}</b>
            <div class="grid gap-2 h-100%" style={{"grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, props.selectedFields.length)}, 1fr)`}}>
                <For each={props.selectedFields}>
                    {(fieldInPacket: GraphStruct) =>
                        <div class="relative">
                            <SolidChart fieldInPacket={fieldInPacket} />
                        </div>
                    }
                </For>
                <Show when={props.selectedFields.length === 0}>
                    <span class="inline-flex items-center justify-center">No packets to display</span>
                </Show>
            </div>
        </div>
    );
};

export default ExpandedFieldsModal;