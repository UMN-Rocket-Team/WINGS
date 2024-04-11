import { Component, createEffect, createSignal } from "solid-js";
import { ReadoutStruct } from "../modals/ReadoutSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import { parsedPackets } from "../backend_interop/buffers";

const Readout: Component<ReadoutStruct> = (readout) => {
    const { parsedPacketCount, PacketStructureViewModels } = useBackend();
    const [value, setValue] = createSignal(0);

    const update = () => {
        const packetData = parsedPackets[readout.packetID];
        if (parsedPackets[readout.packetID] === undefined) {
            parsedPackets[readout.packetID] = [];
        }

        const lastPacket = packetData[packetData.length - 1];
        const latestValue = lastPacket.fieldData[readout.fieldIndex];
        setValue(latestValue);
    };

    createEffect(() => {
        // Update this effect whenever the parsed packet count changes
        const _ = parsedPacketCount();
        update();
    });

    update();

    return (
        <div>
            {value()}
        </div>
    );
};

export default Readout;
