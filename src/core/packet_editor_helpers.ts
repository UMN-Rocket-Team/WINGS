import { Accessor, Component } from "solid-js";
import { PacketComponent, PacketDelimiter, PacketField } from "../backend_interop/types";
import ErrorModal, { ErrorModalProps } from "../modals/ErrorModal";
import { ModalProps } from "../core/ModalProvider";

/**
 * Creates a generic function that will invoke a backend API setter function. This function is defined in a `.ts` file separate from `PacketsTab.tsx` since `.tsx` syntax interferes
 * with that for generic function parameters. If an error occurs inside the API setter, an error modal is shown.
 * 
 * @param selectedPacketStructureID a getter for the current selected packet structure ID
 * @param selectedPacketStructureComponent a getter for the current selected packet structure component
 * @param showModal a function that will show an error modal
 * @returns a function that will call the given API setter on the current selected packet structure component with the given value
 */
export const createInvokeApiSetterFunction = (selectedPacketStructureID: Accessor<number | null>, selectedPacketStructureComponent: Accessor<PacketComponent | null>, showModal: (component: Component<ModalProps<ErrorModalProps>>, modalProps: ErrorModalProps) => void) => {
    return async <T>(apiSetter: (packetStructureId: number, fieldIndex: number, value: T) => Promise<unknown>, value: T) => {
        try {
            await apiSetter(selectedPacketStructureID()!, (selectedPacketStructureComponent()!.data as PacketField | PacketDelimiter).index , value);//dont call this function with a PacketGap and it will be fine
        } catch (error) {
            showModal(ErrorModal, {
                error: "Failed to modify value",
                description: `${error}`
            });
        }
    };
};