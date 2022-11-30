import { Component, createSignal, For } from "solid-js";
import SolidChart from "./SolidChart";

type PacketFieldIds = {
    packetId: number;
    fieldIndex: number;
};

const GraphScreen: Component = () => {
    const [packets, setPackets] = createSignal<PacketFieldIds[]>([
        { packetId: 0, fieldIndex: 0 },
        { packetId: -100, fieldIndex: 99 },
    ]);

    return (
        <div class="flex border-rounded border-transparent bg-gray">
            <For each={packets()}>
                {(ids) =>
                    <div class="flex">
                        <span>{ids.packetId}</span>
                        <span>{ids.fieldIndex}</span>
                    </div>
                }
            </For>
            <SolidChart />
        </div>
    );
};

export default GraphScreen;