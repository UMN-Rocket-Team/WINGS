import { Component, createSignal, For } from "solid-js";
import { useBackendInteropManager } from "./BackendInteropManagerProvider";

type PacketFieldIds = {
    packetId: number;
    fieldIndex: number;
};

const GraphScreen: Component = () => {
    const { packetStructures } = useBackendInteropManager();

    const [packets, setPackets] = createSignal<PacketFieldIds[]>([
        { packetId: 0, fieldIndex: 0 },
    ]);

    return (
        <div class="flex border-rounded border-transparent bg-gray">
            <For each={packets()}>
                {(ids) =>
                    <div class="flex">
                        <span>{packetStructures[ids.packetId].name}</span>
                        <span>{packetStructures[ids.packetId].fields[ids.fieldIndex].name}</span>
                    </div>
                }
            </For>
        </div>
    );
};

export default GraphScreen;