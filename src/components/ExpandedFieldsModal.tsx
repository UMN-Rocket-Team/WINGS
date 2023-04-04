import { ModalProps } from "./ModalProvider";
import { For, JSX } from "solid-js";
import { FieldInPacket, FieldsViewState } from "./FieldsView";
import SolidChart from "./SolidChart";

const ExpandedFieldsModal = (props: ModalProps<FieldsViewState>): JSX.Element => {
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
            <button class="absolute right-4 top-4 p-1 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-900 border-rounded aspect-square"
                onClick={() => props.closeModal({})}>
                {/* bx:x */}
                <svg xmlns="http://www.w3.org/2000/svg" class="dark:text-white" width={28} preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="currentColor" d="m16.192 6.344l-4.243 4.242l-4.242-4.242l-1.414 1.414L10.535 12l-4.242 4.242l1.414 1.414l4.242-4.242l4.243 4.242l1.414-1.414L13.364 12l4.242-4.242z" /></svg>
            </button>
            <b class="text-center text-4xl dark:text-white">{`Screen ${props.number}`}</b>
            <div class="grid gap-2 h-100%" style={{"grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, props.fieldsInPackets.length)}, 1fr)`}}>
                <For each={props.fieldsInPackets}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="relative">
                            <SolidChart fieldInPacket={fieldInPacket} />
                        </div>
                    }
                </For>
            </div>
        </div>
    );
};

export default ExpandedFieldsModal;