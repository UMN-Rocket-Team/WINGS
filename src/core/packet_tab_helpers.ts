import { Accessor } from "solid-js";
import { PacketComponent } from "../backend_interop/types";

export const createInvokeApiSetterFunction = (selectedPacketStructureIndex: Accessor<number | null>, selectedPacketStructureComponent: Accessor<PacketComponent | null>) => {
    return <T>(apiSetter: (packetStructureId: number, fieldIndex: number, value: T) => void, value: T) => {
        apiSetter(selectedPacketStructureIndex()!, selectedPacketStructureComponent()!.data.index, value);
    };
};