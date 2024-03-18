import { JSX, ParentComponent } from "solid-js";
import closeIcon from "../assets/close.svg";

/**
 * The properties required for the {@link DefaultModalLayout} component.
 */
export type DefaultModalLayoutProps = {
    /**
     * A function that closes the current modal.
     */
    close: () => void,
    /**
     * The title of this modal. Nothing will be displayed by default.
     */
    title?: string;
};

/**
 * A component that positions its child elements so that they appear in a good-looking, modal way. The modal will close
 * when the user clicks outside of the modal, presses `Escape`, or clicks the close button.
 * 
 * @param props the properties parameter to this component containing a close function and title
 */
const DefaultModalLayout: ParentComponent<DefaultModalLayoutProps> = (props): JSX.Element => {   
    return (
        <div class="absolute z-10 top-0 left-0 bottom-0 right-0 flex" tabIndex={-1} 
                // Focus the root div of the modal when it is made visible so that it receives keyboard events.
                // The root div of the modal needs to receive keyboard events so that it can close when the Escape key is pressed
                ref={rootElement => setTimeout(() => rootElement.focus())} // Not sure why the setTimeout is necessary, but it is
                onKeyDown={event => {
                    // Close the modal if the Escape key is pressed
                    if ((event.key || event.code) === "Escape") {
                        props.close();
                    }
                }}
                // Close the modal if the user clicks outside
                onClick={() => props.close()}>
            <div class="flex flex-col items-center ma max-h-75% p-4 gap-4 bg-white dark:bg-dark-700 border-rounded border-2 border-gray-200 dark:border-dark-900 relative"
                    // Do not close the modal if the user clicks inside
                    onClick={event => event.stopPropagation()}>
                <button class="absolute w-6 h-6 right-4 top-4 p-1 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-900 border-rounded aspect-square"
                        onClick={() => props.close()}>
                    <img alt="Close" src={closeIcon} class="w-full h-full dark:invert" draggable={false} />
                </button>
                <b class="text-center text-4xl dark:text-white mx-14">{props.title}</b>
                <div class="overflow-auto">
                    {props.children}
                </div>
            </div>
        </div>
    );
};

export default DefaultModalLayout;