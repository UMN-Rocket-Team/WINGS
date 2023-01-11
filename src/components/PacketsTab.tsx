import { batch, Component, createMemo, createSignal, For, Match, Switch } from "solid-js";
import { PacketDelimiter, PacketField, PacketFieldType, PacketGap, PacketMetadataType, PacketStructure } from "../backend_interop/types";
import { useBackendInteropManager } from "./BackendInteropManagerProvider";

type PacketComponent = {
    type: "field" | "delimiter" | "gap";
    data: PacketField | PacketDelimiter | PacketGap;
};

const getSizeOfType = (type: PacketFieldType): number => {
    switch (type) {
        case PacketFieldType.UnsignedByte:
        case PacketFieldType.SignedByte:
            return 1;
        case PacketFieldType.UnsignedShort:
        case PacketFieldType.SignedShort:
            return 2;
        case PacketFieldType.UnsignedInteger:
        case PacketFieldType.SignedInteger:
            return 4;
        case PacketFieldType.UnsignedLong:
        case PacketFieldType.SignedLong:
            return 8;
        case PacketFieldType.Float:
            return 4;
        case PacketFieldType.Double:
            return 8;
    }
};

const getPacketComponents = (packetStructure: PacketStructure): PacketComponent[] => {
    const delimiters: PacketComponent[] = packetStructure.delimiters.map(delimiter => ({ type: "delimiter", data: delimiter }));
    const fields: PacketComponent[] = packetStructure.fields.map(field => ({ type: "field", data: field }));
    const fieldsAndDelimiters: PacketComponent[] = fields.concat(...delimiters).sort((lhs, rhs) => (lhs.data as PacketField | PacketDelimiter).offsetInPacket - (rhs.data as PacketField | PacketDelimiter).offsetInPacket);

    for (var i = 0; i < fieldsAndDelimiters.length - 1; ++i) {
        const size = fieldsAndDelimiters[i].type === "field"
            ? getSizeOfType((fieldsAndDelimiters[i].data as PacketField).type)
            : (fieldsAndDelimiters[i].data as PacketDelimiter).identifier.length;

        const startIndex = (fieldsAndDelimiters[i].data as PacketField | PacketDelimiter).offsetInPacket;
        const nextStartIndex = (fieldsAndDelimiters[i + 1].data as PacketField | PacketDelimiter).offsetInPacket;

        if (startIndex + size < nextStartIndex) {
            fieldsAndDelimiters.splice(i + 1, 0, { type: "gap", data: { size: nextStartIndex - startIndex - size } });
            ++i;
        }
    }

    return fieldsAndDelimiters;
};

type PacketViewModel = {
    name: string;
    components: PacketComponent[]
};

const PacketsTab: Component = () => {
    const { packetStructures, setFieldName, setFieldType } = useBackendInteropManager();

    const packetComponents: () => PacketViewModel[] = createMemo(() => Object.values(packetStructures).map(packetStructure => ({ name: packetStructure.name, components: getPacketComponents(packetStructure) })));

    const initialPacketStructureCount = Object.keys(packetComponents()).length;
    const [selectedPacketStructureIndex, setSelectedPacketStructureIndex] = createSignal<number | null>(initialPacketStructureCount === 0 ? null : 0);
    const [selectedPacketComponentIndex, setSelectedPacketComponentIndex] = createSignal<number | null>(initialPacketStructureCount === 0 ? null : 0);

    const selectedPacket = createMemo(() => packetComponents()[selectedPacketStructureIndex()!]);
    const selectedPacketStructureComponents = createMemo(() => selectedPacketStructureIndex() === null ? [] : selectedPacket().components);
    const selectedPacketStructureComponent = createMemo(() => selectedPacketComponentIndex() === null ? null : selectedPacketStructureComponents()[selectedPacketComponentIndex()!]);
    const selectedFieldData = createMemo(() => selectedPacketStructureComponent()?.type === "field" ? selectedPacketStructureComponent()?.data as PacketField : null);
    const selectedDelimiterData = createMemo(() => selectedPacketStructureComponent()?.type === "delimiter" ? selectedPacketStructureComponent()?.data as PacketDelimiter : null);
    const selectedGapData = createMemo(() => selectedPacketStructureComponent()?.type === "gap" ? selectedPacketStructureComponent()?.data as PacketGap : null);

    return (
        <div class="flex gap-2">
            <div class="flex flex-col gap-2">
                <div class="flex flex-col flex-grow border-1 p-2 border-rounded gap-2 dark:text-white">
                    <h1 class="m-0">Packets</h1>
                    <For each={packetComponents()}>
                        {(packetStructure, i) => (
                            <button class={`flex justify-between gap-4 border-black dark:border-white ${selectedPacketStructureIndex() === i() ? "border-transparent bg-blue-600 text-white" : "bg-transparent"} border-rounded border-1 px-2 py-2 dark:text-white`} onClick={() => batch(() => {
                                setSelectedPacketStructureIndex(i());
                                setSelectedPacketComponentIndex(0);
                            })}>
                                <span class="" style={{ "white-space": "nowrap" }}>{packetStructure.name}</span>
                            </button>
                        )}
                    </For>
                </div>
                <button>Import Packet...</button>
                <button>Export Packet...</button>
                <button>Add Empty Packet</button>
            </div>
            <div class="flex flex-col gap-2">
                <div class="flex flex-col justify-between flex-grow border-1 p-2 border-rounded gap-2 dark:b-white">
                    <div class="flex flex-col flex-grow gap-2 dark:text-white">
                        <h2 class="m-0">{selectedPacket().name}</h2>
                        <span>Components</span>
                        <For each={selectedPacketStructureComponents()}>
                            {(component, i) => (
                                <button class={`flex justify-between gap-4 border-black ${selectedPacketComponentIndex() === i() ? "border-white bg-blue-600 text-white" : "bg-transparent dark:border-white"} border-rounded border-1 px-2 py-2 dark:text-white`} onClick={() => setSelectedPacketComponentIndex(i())}>
                                    <Switch>
                                        <Match when={component.type === "field"}>
                                            <span>F</span>
                                            <span>{(component.data as PacketField).name}</span>
                                        </Match>
                                        <Match when={component.type === "delimiter"}>
                                            <span>D</span>
                                            <span>{(component.data as PacketDelimiter).name}</span>
                                        </Match>
                                        <Match when={component.type === "gap"}>
                                            <span>G</span>
                                            <span>{(component.data as PacketGap).size} Byte Gap</span>
                                        </Match>
                                    </Switch>
                                </button>
                            )}
                        </For>
                    </div>
                    <button class="bg-red border-rounded border-0 px-4 py-2">
                        Delete {selectedPacket().name}
                    </button>
                </div>
                <div class="flex gap-2">
                    <button>Add Field</button>
                    <button>Add Delimeter</button>
                    <button>Add Gap</button>
                    <button>Remove Item</button>
                </div>
            </div>
            <div class="flex flex-col justify-between border-1 p-2 border-rounded dark:b-white">
                <div class="flex flex-col dark:text-white">
                    <Switch>
                        <Match when={selectedPacketComponentIndex() === null}>

                        </Match>
                        <Match when={selectedFieldData() !== null}>
                            <h2 class="m-0">Field Information</h2>
                            <div class="flex flex-col gap-2">
                                <div class="flex flex-col">
                                    <label for="fieldName">Name</label>
                                    <input type="text" value={selectedFieldData()!.name} id="fieldName"
                                        onInput={e => setFieldName(selectedPacketStructureIndex()!, selectedPacketComponentIndex()!, (e.target as HTMLInputElement).value)} />
                                </div>
                                <span>Offset in Packet: {selectedFieldData()!.offsetInPacket} byte{selectedFieldData()!.offsetInPacket == 1 ? "" : "s"}</span>
                                <div class="flex flex-col">
                                    <label for="fieldType">Type</label>
                                    <select value={selectedFieldData()!.type} id="fieldType" 
                                            onInput={e => setFieldType(selectedPacketStructureIndex()!, selectedPacketComponentIndex()!, (e.target as HTMLSelectElement).value as PacketFieldType)}>
                                        <For each={Object.values(PacketFieldType).filter(k => isNaN(Number(k)))}>
                                            {(fieldType) => <option value={fieldType}>{fieldType}</option>}
                                        </For>
                                    </select>
                                </div>
                                <div class="flex flex-col">
                                    <label for="fieldMetadataType">Metadata Type</label>
                                    <select value={selectedFieldData()!.metadataType} id="fieldMetadataType">
                                        <For each={Object.values(PacketMetadataType).filter(k => isNaN(Number(k)))}>
                                            {(metadataType) => <option value={metadataType}>{metadataType}</option>}
                                        </For>
                                    </select>
                                </div>
                            </div>
                        </Match>
                        <Match when={selectedDelimiterData() !== null}>
                            <h2 class="m-0">Delimiter Information</h2>
                            <div class="flex flex-col gap-2">
                                <div class="flex flex-col">
                                    <span>Name</span>
                                    <input type="text" value={selectedDelimiterData()!.name} />
                                </div>
                                <span>Offset in Packet: {selectedDelimiterData()!.offsetInPacket} byte{selectedDelimiterData()!.offsetInPacket == 1 ? "" : "s"}</span>
                            </div>
                        </Match>
                        <Match when={selectedGapData() !== null}>
                            <h2 class="m-0">Gap Information</h2>
                            <div class="flex flex-col">
                                <span>Size</span>
                                <input type="number" value={selectedGapData()!.size} min={1} onChange={(e) => {
                                    const value = e.currentTarget.value;

                                    if (value.match('^[0-9]*$')) {
                                        selectedGapData()!.size = +value;
                                        return value;
                                    }
                                    return selectedGapData()!.size;
                                }} />
                            </div>
                        </Match>
                    </Switch>
                </div>
                <button class="bg-red border-rounded border-0 px-4 py-2">
                    Delete {(selectedPacketStructureComponent()?.data as any).name ?? "Gap"}
                </button>
            </div>
        </div>
    );
};

export default PacketsTab;