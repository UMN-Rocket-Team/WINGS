import { createContext, createSignal, JSX, ParentComponent, useContext } from "solid-js";
import { Dynamic, Portal } from "solid-js/web";
import { createShowModalFunction } from "../../core/modal_helpers";

/**
 * The type of the properties passed to all child components used in a modal. 
 * `BaseType` is the type of the properties that will be passed to the child component and are provided by a call to `useModal`.
 * `ResultType` is the type of the result of showing the modal.
 */
export type ModalProps<BaseType = {}, ResultType = {}> = BaseType & {
    closeModal: (result: ResultType) => void,
};

/**
 * The type of metadata used by the modal provider.
 */
export type ModalMetadata<ResultType> = {
    /**
     * A callback called when the modal is closed. `ResultType` is the type of the result of showing the modal.
     */
    modalClosedCallback?: ((result: ResultType) => void),
};

/**
 * The type of the value given when using the `ModalContext`.
 */
export type ModalContextValue = {
    /**
     * Shows the modal with the given child component, child component properties, and metadata.
     * The given component is constructed using the given child component properties. Metadata is
     * used for things other than component creation.
     */
    showModal: <BaseType, ResultType>(component: (props: ModalProps<BaseType, ResultType>) => JSX.Element, modalProps: BaseType & ModalMetadata<ResultType>) => void,
};

/**
 * The context that holds the global {@link ModalContextValue}.
 */
const ModalContext = createContext<ModalContextValue>({
    showModal: (): never => { throw new Error("Cannot show modal in default ModalContext implementation!"); },
});

/**
 * A component that abstracts showing a single modal component on the screen.
 * 
 * @param props the children components to provide the ability to show a modal to
 * @returns a component wrapping the given child component that provides the ability to show a modal
 * @see {@link ModalContextValue} for the provided global function to show modals 
 */
export const ModalProvider: ParentComponent = (props): JSX.Element => {
    const [modalComponent, setModalComponent] = createSignal<(() => JSX.Element) | undefined>(undefined);

    const context: ModalContextValue = {
        showModal: createShowModalFunction(setModalComponent),
    };

    return (
        <ModalContext.Provider value={context}>
            {props.children}
            <Portal>
                <Dynamic component={modalComponent()} />
            </Portal>
        </ModalContext.Provider>
    );
};

/**
 * Use the modal system provided by the global {@link ModalContext}.
 * 
 * @returns the current {@link ModalContextValue}
 */
export const useModal = (): ModalContextValue => useContext(ModalContext);
