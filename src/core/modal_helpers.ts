import { JSX, Setter } from "solid-js";
import { ModalMetadata, ModalProps } from "../components/ModalProvider";

/**
 * Creates a generic function that will show a modal. This function is defined in a `.ts` file separate from `ModalProvider.tsx` since `.tsx` syntax interferes
 * with that for generic function parameters.
 * 
 * @param setModalComponent a setter for the child component of a modal
 * @returns a function that will show a modal with the given component, component properties, and metadata
 */
export const createShowModalFunction = (setModalComponent: Setter<(() => JSX.Element) | null>) => {
    return <BaseType, ResultType>(component: (props: ModalProps<BaseType, ResultType>) => JSX.Element, props: BaseType & ModalMetadata<ResultType>): void => {
        // Note: to call a setter with a value of a function, the overload which takes the previous state must be explicity used
        setModalComponent((_previousModalComponent) => 
            // Wrap the component inside a function that takes no arguments--so the ModalProvider does not need to worry about props,
            // captures the props, and supplies the closeModal function implementation
            () => component({
                closeModal: (result: ResultType) => {
                    setModalComponent(null);
                    props.modalClosedCallback && props.modalClosedCallback(result);
                },
                ...props
            }));
    };
};