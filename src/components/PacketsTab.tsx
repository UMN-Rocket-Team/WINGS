import { batch, Component, createMemo, createSignal, For, Match, Show, Switch } from "solid-js";
import { addDelimiter, addField, addGapAfter, deletePacketStructureComponent, setDelimiterIdentifier, setDelimiterName, setFieldMetadataType, setFieldName, setFieldType, setGapSize, importPacket, exportPacket} from "../backend_interop/api_calls";
import { PacketComponentType, PacketDelimiter, PacketField, PacketFieldType, PacketGap, PacketMetadataType } from "../backend_interop/types";
import { createInvokeApiSetterFunction } from "../core/packet_tab_helpers";
import { useBackendInteropManager } from "./BackendInteropManagerProvider";

const PacketsTab: Component = () => {
    const { packetViewModels } = useBackendInteropManager();

    const [selectedPacketStructureIndex, setSelectedPacketStructureIndex] = createSignal<number | null>(packetViewModels.length === 0 ? null : 0);
    const [selectedPacketComponentIndex, setSelectedPacketComponentIndex] = createSignal<number | null>(packetViewModels.length === 0 ? null : 0);

    const selectedPacket = createMemo(() => packetViewModels[selectedPacketStructureIndex()!]);
    const selectedPacketStructureComponents = createMemo(() => selectedPacketStructureIndex() === null ? [] : selectedPacket().components);
    const selectedPacketStructureComponent = createMemo(() => selectedPacketComponentIndex() === null ? null : selectedPacketStructureComponents()[selectedPacketComponentIndex()!]);
    const selectedFieldData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Field ? selectedPacketStructureComponent()?.data as PacketField : null);
    const selectedDelimiterData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Delimiter ? selectedPacketStructureComponent()?.data as PacketDelimiter : null);
    const selectedGapData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Gap ? selectedPacketStructureComponent()?.data as PacketGap : null);

    const invokeApiSetter = createInvokeApiSetterFunction(selectedPacketStructureIndex, selectedPacketStructureComponent);

    return (
        <div class="flex gap-2">
            <div class="flex flex-col gap-2">
                <div class="flex flex-col flex-grow tab">
                    <h1 class="m-0">Packets</h1>
                    <For each={packetViewModels}>
                        {(packetStructure, i) => (
                            <button class={`flex justify-between gap-4 ${selectedPacketStructureIndex() === i() ? "widgetSelected" : "widgetNotSelected"} widgetGeneral`} onClick={() => batch(() => {
                                setSelectedPacketStructureIndex(i());
                                setSelectedPacketComponentIndex(0);
                            })}>
                                <span class="" style={{ "white-space": "nowrap" }}>{packetStructure.name}</span>
                            </button>
                        )}
                    </For>
                </div>
                <button class="externalButton" onClick={e => importPacket()}>Import Packet...</button>
                <button class="externalButton" onClick={e => exportPacket(selectedPacket())}>Export Packet...</button>
                <button class="externalButton" onClick={e => addEmptyPacket()}>Add Empty Packet</button>
            </div>
            <div class="flex flex-col gap-2">
                <div class="flex flex-col justify-between flex-grow tab">
                    <Show when={selectedPacketStructureIndex() !== null} fallback={<h2 class="m-0 dark:text-white">No packet selected</h2>}>
                        <div class="flex flex-col flex-grow gap-2 dark:text-white">
                            <h2 class="m-0">{selectedPacket()!.name}</h2>
                            <span>Components</span>
                            <For each={selectedPacketStructureComponents()}>
                                {(component, i) => (
                                    <button class={`flex justify-between gap-4 ${selectedPacketComponentIndex() === i() ? "widgetSelected" : "widgetNotSelected"} widgetGeneral`} onClick={() => setSelectedPacketComponentIndex(i())}>
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
                        <button class="redButton" onClick={() => deletePacketStructure()}>
                            Delete {selectedPacket()!.name}
                        </button>
                    </Show>
                </div>
                <div class="flex gap-2">
                    <button class="externalButton" onClick={() => addField(selectedPacketStructureIndex()!)}>Add Field</button>
                    <button class="externalButton" onClick={() => addDelimiter(selectedPacketStructureIndex()!)}>Add Delimeter</button>
                    <button class="externalButton" onClick={() => {
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
            <div class="flex flex-col justify-between tab">
                <Show when={selectedPacketStructureIndex() !== null} fallback={<h2 class="m-0 dark:text-white">No component selected</h2>}>
                    <div class="flex flex-col dark:text-white">
                        <Switch>
                            <Match when={selectedPacketComponentIndex() === null}>

                            </Match>
                            <Match when={selectedFieldData() !== null}>
                                <h2 class="m-0">Field Information</h2>
                                <div class="flex flex-col gap-2">
                                    <div class="flex flex-col">
                                        <label for="fieldName">Name</label>
                                        <input class="inputBox" type="text" value={selectedFieldData()!.name} id="fieldName"
                                            onInput={e => invokeApiSetter(setFieldName, (e.target as HTMLInputElement).value)} />
                                    </div>
                                    <span>Offset in Packet: {selectedFieldData()!.offsetInPacket} byte{selectedFieldData()!.offsetInPacket == 1 ? "" : "s"}</span>
                                    <div class="flex flex-col">
                                        <label for="fieldType">Type</label>
                                        <select class="inputBox" value={selectedFieldData()!.type} id="fieldType"
                                            onInput={e => invokeApiSetter(setFieldType, ((e.target as HTMLSelectElement).value as PacketFieldType))}>
                                            <For each={Object.values(PacketFieldType).filter(k => isNaN(Number(k)))}>
                                                {(fieldType) => <option value={fieldType}>{fieldType}</option>}
                                            </For>
                                        </select>
                                    </div>
                                    <div class="flex flex-col">
                                        <label for="fieldMetadataType">Metadata Type</label>
                                        <select class="inputBox" value={selectedFieldData()!.metadataType} id="fieldMetadataType"
                                            onInput={e => invokeApiSetter(setFieldMetadataType, (e.target as HTMLSelectElement).value as PacketMetadataType)}>
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
                                        <label for="delimiterName">Name</label>
                                        <input class="inputBox" type="text" value={selectedDelimiterData()!.name} id="delimiterName"
                                            onInput={e => invokeApiSetter(setDelimiterName, (e.target as HTMLInputElement).value)} />
                                    </div>
                                    <div>
                                        <label for="delimiterIdentifier">Identifier:</label>
                                        <input class="inputBox" type="text" value={selectedDelimiterData()!.identifier} id="delimiterIdentifier"
                                            onInput={e => {
                                                (e.target as HTMLInputElement).value = (e.target as HTMLInputElement).value.replace(/[^\da-f]/g, '');

                                                invokeApiSetter(setDelimiterIdentifier, (e.target as HTMLInputElement).value);
                                            }} />
                                    </div>
                                    <span>Offset in Packet: {selectedDelimiterData()!.offsetInPacket} byte{selectedDelimiterData()!.offsetInPacket == 1 ? "" : "s"}</span>
                                </div>
                            </Match>
                            <Match when={selectedGapData() !== null}>
                                <h2 class="m-0">Gap Information</h2>
                                <div class="flex flex-col">
                                    <label for="gapSize">Size</label>
                                    <input class="inputBox" type="number" value={selectedGapData()!.size} min={1} id="gapSize" onChange={(e) => {
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
                    <button class="redButton" onClick={() => invokeApiSetter(deletePacketStructureComponent, selectedPacketStructureComponent()!.type)}>
                        Delete {(selectedPacketStructureComponent()?.data as any).name ?? "Gap"}
                    </button>
                </Show>
            </div>
        </div>
    );
};

export default PacketsTab;