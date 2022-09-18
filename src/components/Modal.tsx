import { JSX, ParentComponent, Show, Accessor, Setter, createEffect, on } from "solid-js";
import { Portal } from "solid-js/web";

export type ModalProps = {
    isVisible: Accessor<boolean>;
    setVisible: Setter<boolean>;
    title?: string;
}

const Modal: ParentComponent<ModalProps> = (props): JSX.Element => {
    let rootElement: HTMLDivElement | undefined = undefined;

    // Focus the root div of the modal when it is made visible so that it receives keyboard events.
    // The root div of the modal needs to receive keyboard events so that it can close when the Escape key is pressed
    createEffect(on(props.isVisible, (isVisible) => {
        if (isVisible) {
            rootElement?.focus();
        }
    }, { defer: true }));

    return (
        <Show when={props.isVisible()}>
            <Portal>
                <div class="absolute z-10 top-0 left-0 bottom-0 right-0 flex" tabIndex={-1} ref={rootElement}
                        onKeyDown={event => {
                            // Close the modal if the Escape key is pressed
                            if ((event.key || event.code) === "Escape") {
                                props.setVisible(false);
                            }
                        }}
                        // Close the modal if the user clicks outside
                        onClick={() => props.setVisible(false)}>
                    <div class="flex flex-col items-center ma max-h-75% p-4 gap-4 bg-white dark:bg-dark-700 border-rounded border-2 border-gray-200 dark:border-dark-900 relative"
                            // Do not close the modal if the user clicks inside
                            onClick={event => event.stopPropagation()}>
                        <button class="absolute right-4 top-4 p-1 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-900 border-rounded aspect-square"
                                onClick={() => props.setVisible(false)}>
                            {/* bx:x */}
                            <svg xmlns="http://www.w3.org/2000/svg" class="dark:text-white" width={28} preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="currentColor" d="m16.192 6.344l-4.243 4.242l-4.242-4.242l-1.414 1.414L10.535 12l-4.242 4.242l1.414 1.414l4.242-4.242l4.243 4.242l1.414-1.414L13.364 12l4.242-4.242z"/></svg>
                        </button>
                        <b class="text-center text-4xl dark:text-white">{props.title}</b>
                        <div class="overflow-scroll">
                            {props.children}
                        </div>
                    </div>
                </div>
            </Portal>
        </Show>
    );
};

export default Modal;