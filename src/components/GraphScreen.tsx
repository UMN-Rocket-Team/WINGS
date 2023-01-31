import { Component, createSignal, For } from "solid-js";
import { useBackendInteropManager } from "./BackendInteropManagerProvider";

type PacketFieldIds = {
    packetId: number;
    fieldIndex: number;
};

const GraphScreen: Component = () => {
    const { packetViewModels } = useBackendInteropManager();

    const [packets, setPackets] = createSignal<PacketFieldIds[]>([
    ]);

    return (
        <div class="flex border-rounded border-transparent bg-gray">
            <For each={packets()}>
                {(ids) =>
                    <div class="flex">
                        <span>{packetViewModels[ids.packetId].name}</span>
                        <span>{packetViewModels[ids.packetId].components[ids.fieldIndex].type}</span>
                    </div>
                }
            </For>
        </div>
    );
};

export default GraphScreen;