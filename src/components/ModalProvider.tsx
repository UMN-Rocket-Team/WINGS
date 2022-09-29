import { createContext, createSignal, JSX, ParentComponent, Show, useContext } from "solid-js";
import { Portal } from "solid-js/web";
import { createShowModalFunction } from "../core/modal_helpers";

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
     * The given component is constructed using the given child component properties. Metatdata is
     * used for things other than component creation.
     */
    showModal: <BaseType, ResultType>(component: (props: ModalProps<BaseType, ResultType>) => JSX.Element, modalProps: BaseType & ModalMetadata<ResultType>) => void, 
};

const ModalContext = createContext<ModalContextValue>({
    showModal: (): never => { throw new Error("Cannot show modal in default ModalContext implementation!"); },
});

export const ModalProvider: ParentComponent = (props): JSX.Element => {
    const [modalComponent, setModalComponent] = createSignal<JSX.Element | null>(null);

    const isVisible = (): boolean => modalComponent() !== null;

    const context: ModalContextValue = {
        showModal: createShowModalFunction(setModalComponent),
    };

    return (
        <ModalContext.Provider value={context}>
            {props.children}
            <Show when={isVisible}>
                <Portal>
                    {modalComponent()}
                </Portal>
            </Show>
        </ModalContext.Provider>
    );
};

export const useModal = (): ModalContextValue => useContext(ModalContext);
