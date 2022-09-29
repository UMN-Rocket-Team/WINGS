import { JSX, Setter } from "solid-js";
import { ModalMetadata, ModalProps } from "../components/ModalProvider";

/**
 * Creates a generic function that will show a modal. This function is defined in a `.ts` file separate from `ModalProvider.tsx` since `.tsx` syntax interferes
 * with that for generic function parameters.
 * 
 * @param setModalComponent a setter for the child component of a modal
 * @returns a function that will show a modal with the given component, component properties, and metadata
 */
export const createShowModalFunction = (setModalComponent: Setter<JSX.Element | null>) => {
    return <BaseType, ResultType>(component: (props: ModalProps<BaseType, ResultType>) => JSX.Element, props: BaseType & ModalMetadata<ResultType>): void => {
        setModalComponent(component({
            closeModal: (result: ResultType) => {
                setModalComponent(null);
                props.modalClosedCallback && props.modalClosedCallback(result);
            }, ...props }));
    };
};