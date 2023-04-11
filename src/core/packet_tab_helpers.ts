import { Accessor } from "solid-js";
import { PacketComponent } from "../backend_interop/types";

/**
 * Creates a generic function that will invoke a backend API setter function. This function is defined in a `.ts` file separate from `PacketsTab.tsx` since `.tsx` syntax interferes
 * with that for generic function parameters.
 * 
 * @param selectedPacketStructureIndex a getter for the current selected packet structure index
 * @param selectedPacketStructureComponent a getter for the current selected packet structure component
 * @returns a function that will call the given API setter on the curent selected packet stucture component with the given value
 */
export const createInvokeApiSetterFunction = (selectedPacketStructureIndex: Accessor<number | null>, selectedPacketStructureComponent: Accessor<PacketComponent | null>) => {
    return <T>(apiSetter: (packetStructureId: number, fieldIndex: number, value: T) => void, value: T) => {
        apiSetter(selectedPacketStructureIndex()!, selectedPacketStructureComponent()!.data.index, value);
    };
};