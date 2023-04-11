import { batch, Component, createMemo, createSignal, For, Match, Show, Switch } from "solid-js";
import { addDelimiter, addField, addGapAfter, deletePacketStructureComponent, setDelimiterIdentifier, setDelimiterName, setFieldMetadataType, setFieldName, setFieldType, setGapSize } from "../backend_interop/api_calls";
import { PacketComponentType, PacketDelimiter, PacketField, PacketFieldType, PacketGap, PacketMetadataType } from "../backend_interop/types";
import { createInvokeApiSetterFunction } from "../core/packet_tab_helpers";
import { useBackend } from "./BackendProvider";

/**
 * A component that allows the user to manage packet structures. Changes on the frontend are synchronized with the Rust
 * packet structure manager backend. 
 * 
 * The user can:
 * - Add an empty packet structure
 * - Save a packet structure to a file
 * - Load a packet structure from a file
 * - Modify a packet structure by:
 *   - Changing its name
 *   - Adding fields
 *   - Adding delimiters
 *   - Adding gaps
 *   - Deleting fields, delimiters, and gaps
 * - Modify a field by:
 *   - Changing its name
 *   - Changing its type
 *   - Modifying its metadata type
 * - Modify a delimiter by:
 *   - Changing its name
 *   - Changing its identifier
 * - Modify a gap by:
 *   - Changing its size
 * - Delete a packet structure
 */
const PacketsTab: Component = () => {
    const { packetViewModels } = useBackend();

    const [selectedPacketStructureIndex, setSelectedPacketStructureIndex] = createSignal<number | null>(packetViewModels.length === 0 ? null : 0);
    const [selectedPacketComponentIndex, setSelectedPacketComponentIndex] = createSignal<number | null>(packetViewModels.length === 0 ? null : 0);

    // Cache (memoize) revelent information about the selected packet
    const selectedPacket = createMemo(() => packetViewModels[selectedPacketStructureIndex()!]);
    const selectedPacketStructureComponents = createMemo(() => selectedPacketStructureIndex() === null ? [] : selectedPacket().components);
    const selectedPacketStructureComponent = createMemo(() => selectedPacketComponentIndex() === null ? null : selectedPacketStructureComponents()[selectedPacketComponentIndex()!]);
    const selectedFieldData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Field ? selectedPacketStructureComponent()?.data as PacketField : null);
    const selectedDelimiterData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Delimiter ? selectedPacketStructureComponent()?.data as PacketDelimiter : null);
    const selectedGapData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Gap ? selectedPacketStructureComponent()?.data as PacketGap : null);

    const invokeApiSetter = createInvokeApiSetterFunction(selectedPacketStructureIndex, selectedPacketStructureComponent);

    return (
        <div class="flex gap-2">
            {/* Packet structure list */}
            <div class="flex flex-col gap-2">
                <div class="flex flex-col flex-grow border-1 p-2 border-rounded gap-2 dark:text-white">
                    <h1 class="m-0">Packets</h1>
                    <For each={packetViewModels}>
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
                <button onClick={e => importPacket()}>Import Packet...</button>
                <button onClick={e => exportPacket()}>Export Packet...</button>
                <button onClick={e => addEmptyPacket()}>Add Empty Packet</button>
            </div>
            {/* Packet structure component list */}
            <div class="flex flex-col gap-2">
                <div class="flex flex-col justify-between flex-grow border-1 p-2 border-rounded gap-2 dark:b-white">
                    <Show when={selectedPacketStructureIndex() !== null} fallback={<h2 class="m-0">No packet selected</h2>}>
                        <div class="flex flex-col flex-grow gap-2 dark:text-white">
                            <h2 class="m-0">{selectedPacket()!.name}</h2>
                            <span>Components</span>
                            <For each={selectedPacketStructureComponents()}>
                                {(component, i) => (
                                    <button class={`flex justify-between gap-4 border-black ${selectedPacketComponentIndex() === i() ? "border-white bg-blue-600 text-white" : "bg-transparent dark:border-white"} border-rounded border-1 px-2 py-2 dark:text-white`} onClick={() => setSelectedPacketComponentIndex(i())}>
                                        <Switch>
                                            <Match when={component.type === PacketComponentType.Field}>
                                                <span>F</span>
                                                <span>{(component.data as PacketField).name}</span>
                                            </Match>
                                            <Match when={component.type === PacketComponentType.Delimiter}>
                                                <span>D</span>
                                                <span>{(component.data as PacketDelimiter).name}</span>
                                            </Match>
                                            <Match when={component.type === PacketComponentType.Gap}>
                                                <span>G</span>
                                                <span>{(component.data as PacketGap).size} Byte Gap</span>
                                            </Match>
                                        </Switch>
                                    </button>
                                )}
                            </For>
                        </div>
                        <button class="bg-red border-rounded border-0 px-4 py-2" onClick={() => deletePacketStructure()}>
                            Delete {selectedPacket()!.name}
                        </button>
                    </Show>
                </div>
                <div class="flex gap-2">
                    <button onClick={() => addField(selectedPacketStructureIndex()!)}>Add Field</button>
                    <button onClick={() => addDelimiter(selectedPacketStructureIndex()!)}>Add Delimeter</button>
                    <button onClick={() => {
                        const selectedComponentType = selectedPacketStructureComponent()!.type;
                        let isField: boolean;
                        let index: number;
                        switch (selectedComponentType) {
                            case PacketComponentType.Field:
                                isField = true;
                                index = selectedFieldData()!.index;
                                break;
                            case PacketComponentType.Delimiter:
                                isField = false;
                                index = selectedDelimiterData()!.index;
                                break;
                            default:
                                throw new Error("Cannot add a gap after a gap!");
                        }
                        addGapAfter(selectedPacketStructureIndex()!, isField, index);
                    }}>Add Gap</button>
                </div>
            </div>
            {/* Packet structure component editor */}
            <div class="flex flex-col justify-between border-1 p-2 border-rounded dark:b-white">
                <Show when={selectedPacketStructureIndex() !== null} fallback={<h2 class="m-0">No component selected</h2>}>
                    <div class="flex flex-col dark:text-white">
                        <Switch>
                            <Match when={selectedPacketComponentIndex() === null}>
                                {/* Should not happen */}
                            </Match>
                            {/* Selected packet structure field editor */}
                            <Match when={selectedFieldData() !== null}>
                                <h2 class="m-0">Field Information</h2>
                                <div class="flex flex-col gap-2">
                                    <div class="flex flex-col">
                                        <label for="fieldName">Name</label>
                                        <input type="text" value={selectedFieldData()!.name} id="fieldName"
                                            onInput={e => invokeApiSetter(setFieldName, (e.target as HTMLInputElement).value)} />
                                    </div>
                                    <span>Offset in Packet: {selectedFieldData()!.offsetInPacket} byte{selectedFieldData()!.offsetInPacket == 1 ? "" : "s"}</span>
                                    <div class="flex flex-col">
                                        <label for="fieldType">Type</label>
                                        <select value={selectedFieldData()!.type} id="fieldType"
                                            onInput={e => invokeApiSetter(setFieldType, ((e.target as HTMLSelectElement).value as PacketFieldType))}>
                                            <For each={Object.values(PacketFieldType).filter(k => isNaN(Number(k)))}>
                                                {(fieldType) => <option value={fieldType}>{fieldType}</option>}
                                            </For>
                                        </select>
                                    </div>
                                    <div class="flex flex-col">
                                        <label for="fieldMetadataType">Metadata Type</label>
                                        <select value={selectedFieldData()!.metadataType} id="fieldMetadataType"
                                            onInput={e => invokeApiSetter(setFieldMetadataType, (e.target as HTMLSelectElement).value as PacketMetadataType)}>
                                            <For each={Object.values(PacketMetadataType).filter(k => isNaN(Number(k)))}>
                                                {(metadataType) => <option value={metadataType}>{metadataType}</option>}
                                            </For>
                                        </select>
                                    </div>
                                </div>
                            </Match>
                            {/* Selected packet structure delimiter editor */}
                            <Match when={selectedDelimiterData() !== null}>
                                <h2 class="m-0">Delimiter Information</h2>
                                <div class="flex flex-col gap-2">
                                    <div class="flex flex-col">
                                        <label for="delimiterName">Name</label>
                                        <input type="text" value={selectedDelimiterData()!.name} id="delimiterName"
                                            onInput={e => invokeApiSetter(setDelimiterName, (e.target as HTMLInputElement).value)} />
                                    </div>
                                    <div>
                                        <label for="delimiterIdentifier">Identifier:</label>
                                        <input type="text" value={selectedDelimiterData()!.identifier} id="delimiterIdentifier"
                                            onInput={e => {
                                                (e.target as HTMLInputElement).value = (e.target as HTMLInputElement).value.replace(/[^\da-f]/g, '');

                                                invokeApiSetter(setDelimiterIdentifier, (e.target as HTMLInputElement).value);
                                            }} />
                                    </div>
                                    <span>Offset in Packet: {selectedDelimiterData()!.offsetInPacket} byte{selectedDelimiterData()!.offsetInPacket == 1 ? "" : "s"}</span>
                                </div>
                            </Match>
                            {/* Selected packet structure gap editor */}
                            <Match when={selectedGapData() !== null}>
                                <h2 class="m-0">Gap Information</h2>
                                <div class="flex flex-col">
                                    <label for="gapSize">Size</label>
                                    <input type="number" value={selectedGapData()!.size} min={1} id="gapSize" onChange={(e) => {
                                        const value = e.currentTarget.value;

                                        if (value.match('^[0-9]*$')) {
                                            selectedGapData()!.size = +value;
                                            return value;
                                        }
                                        return selectedGapData()!.size;
                                    }} onInput={e => invokeApiSetter(setGapSize, +(e.target as HTMLInputElement).value)} />
                                </div>
                            </Match>
                        </Switch>
                    </div>
                    <button class="bg-red border-rounded border-0 px-4 py-2" onClick={() => invokeApiSetter(deletePacketStructureComponent, selectedPacketStructureComponent()!.type)}>
                        Delete {(selectedPacketStructureComponent()?.data as any).name ?? "Gap"}
                    </button>
                </Show>
            </div>
        </div>
    );
};

export default PacketsTab;