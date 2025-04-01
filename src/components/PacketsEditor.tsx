/* eslint-disable solid/reactivity */
import { batch, Component, createMemo, createSignal, For, Match, Show, Switch } from "solid-js";
import { addDelimiter, addField, addGapAfter, deletePacketStructure, deletePacketStructureComponent, registerEmptyPacketStructure, setDelimiterIdentifier, setDelimiterName, setFieldMetadataType, setFieldName, setFieldType, setGapSize, setPacketName } from "../backend_interop/api_calls";
import { PacketComponentType, PacketDelimiter, PacketField, PacketFieldType, PacketGap, PacketMetadataType } from "../backend_interop/types";
import { createInvokeApiSetterFunction } from "../core/packet_editor_helpers";
import { runExportPacketWindow, importPacketsFromDirectories} from "../core/file_handling";
import { useBackend } from "../backend_interop/BackendProvider";
import { useModal } from "../core/ModalProvider";
import ErrorModal from "../modals/ErrorModal";
import FileModal from "../modals/FilePathModal";
import { Store } from "tauri-plugin-store-api";

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
const PacketEditor: Component = () => {
    const { PacketStructureViewModels } = useBackend();
    const { showModal } = useModal();

    const [selectedPacketStructureID, setSelectedPacketStructureID] = createSignal<number | null>(PacketStructureViewModels.length === 0 ? null : 1);
    const [selectedPacketComponentIndex, setSelectedPacketComponentIndex] = createSignal<number | null>(PacketStructureViewModels.length === 0 ? null : 0);

    const selectedPacket = createMemo(() => PacketStructureViewModels.find(i => i.id === selectedPacketStructureID()) || null);
    const selectedPacketStructureComponents = createMemo(() => selectedPacket() ? selectedPacket()!.components : []);
    const selectedPacketStructureComponent = createMemo(() => selectedPacketComponentIndex() === null ? null : selectedPacketStructureComponents()[selectedPacketComponentIndex()!]);
    const selectedFieldData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Field ? selectedPacketStructureComponent()?.data as PacketField : null);
    const selectedDelimiterData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Delimiter ? selectedPacketStructureComponent()?.data as PacketDelimiter : null);
    const selectedGapData = createMemo(() => selectedPacketStructureComponent()?.type === PacketComponentType.Gap ? selectedPacketStructureComponent()?.data as PacketGap : null);

    const invokeApiSetter = () => createInvokeApiSetterFunction(selectedPacketStructureID, selectedPacketStructureComponent, showModal);

    async function showErrorModalOnError(func: () => Promise<unknown>, errorTitle: string): Promise<void> {
        try {
            await func();
        } catch (error) {
            showModal(ErrorModal, {
                error: errorTitle,
                description: `${error}`
            });
        }
    }

    return (
        <div class="flex gap-2 width-[100%] overflow-hidden">
            {/* Packet structure list */}
            <div class="flex flex-col gap-2">
                <div class="h-[100%] flex flex-col overflow-scroll flex-grow tab">
                    <h1 class="m-0 text-xl font-bold text-black dark:text-white">Packets</h1>
                        
                        <For each={PacketStructureViewModels}>
                            {packetStructure => (
                                <button class={`flex justify-between gap-4 m-1 text-black bg-gray-100 hover:bg-gray-300 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 
                                    dark:text-white
                                    dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 
                                    ${selectedPacketStructureID() === packetStructure.id ? "widgetSelected" : "widgetNotSelected"} widgetGeneral`} onClick={() => batch(() => {
                                    setSelectedPacketStructureID(packetStructure.id);
                                    setSelectedPacketComponentIndex(0);
                                })}>
                                    <span class="" style={{ "white-space": "nowrap" }}>{packetStructure.name}</span>
                                </button>
                            )}
                        </For>
                </div>
                <button class="externalButton m-1 text-black bg-gray-100 hover:bg-gray-300 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 
                    dark:text-white dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700" 
                    onClick={async () => {
                        const store = new Store("persistent.dat");
                        const recentPaths = (await store.get("recentSaves") || []) as string[];
                        showModal(FileModal, {
                            pathStrings: recentPaths,
                            callBack: importPacketsFromDirectories
                    })
                }}>
                    Import Packet
                </button>
                {/*<button class="externalButton" onClick={async () => await runImportPacketWindow()}>Add Packet</button>*/}
                <button class="externalButton m-0 text-black bg-gray-100 hover:bg-gray-300 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 
                    dark:text-white dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700" 
                    onClick={async () => await runExportPacketWindow(selectedPacket()!)}
                >
                        Export Packet...
                </button>
                
                <button class="externalButton m-0 text-black bg-gray-100 hover:bg-gray-300
                    focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm 
                    px-5 py-2.5 dark:text-white dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700
                    dark:border-gray-700" 
                    onClick={async () => await showErrorModalOnError(registerEmptyPacketStructure, 'Failed to add empty packet')}
                >
                    Add Empty Packet
                </button>
            </div>
            {/* Packet structure component list */}
            <div class="flex flex-col justify-between gap-2">
                <div class="flex flex-col flex-grow justify-between overflow-auto contentContainer">
                    <Show when={selectedPacket() !== null} fallback={<h2 class="m-0 dark:text-white">No packet selected</h2>}>
                        <div class="flex-col gap-2 overflow-auto dark:text-white">
                            <h2 class="text-xl font-bold m-0">{selectedPacket()!.name}</h2>
                            <label class='flex flex-col'>
                                <span class="text-gray-700 dark:text-gray-400 text-sm">Name</span>
                                <input class="inputBox" type='text' value={selectedPacket()!.name}
                                    onInput={async e => await showErrorModalOnError(async () => await setPacketName(selectedPacket()!.id, (e.target as HTMLInputElement).value), 'Failed to change packet name')} />
                            </label>
                            <span class="font-bold">Components</span>
                            <div class="justify-between">
                                <div class="flex flex-col gap-2 pb-10">
                                    <For each={selectedPacketStructureComponents()}>
                                        {(component, i) => (
                                            <button class={`flex justify-between gap-4 m-0 text-black bg-gray-100 hover:bg-gray-300 focus:outline-none focus:ring-4 
                                                focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-0 dark:bg-gray-800 dark:hover:bg-gray-700 
                                                dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white
                                                ${selectedPacketComponentIndex() === i() ? "widgetSelected" : "widgetNotSelected"} widgetGeneral`} onClick={() => setSelectedPacketComponentIndex(i())}
                                            >
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
                            </div>
                        </div>
                        <button class="redButton relative bottom-0 pt-2 mt-5 m-0 dark:text-white hover:bg-gray-300 focus:outline-none focus:ring-4 
                        focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 bg-gray-100 text-black
                        dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700"
                            onClick={async () => await showErrorModalOnError(async () => {
                                await deletePacketStructure(selectedPacket()!.id);
                                // Select the previous packet structure if the last packet structure was deleted, select no packet structure
                                // if none are left
                                setSelectedPacketStructureID(PacketStructureViewModels.length === 0 ? null : PacketStructureViewModels[0].id);
                            }, 'Failed to delete packet structure!')}
                        >
                            Delete {selectedPacket()!.name}
                        </button>
                    </Show>
                </div>
                <div class="flex gap-2">
                    <button class="externalButton m-0 text-black bg-gray-100 hover:bg-gray-300 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white" onClick={async () => await showErrorModalOnError(async () => await addField(selectedPacketStructureID()!), 'Failed to add field')}>Add Field</button>
                    <button class="externalButton m-0 text-black bg-gray-100 hover:bg-gray-300 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white" onClick={async () => await showErrorModalOnError(async () => await addDelimiter(selectedPacketStructureID()!), 'Failed to add delimiter')}>Add Delimiter</button>
                    <button class="externalButton m-0 text-black bg-gray-100 hover:bg-gray-300 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white" onClick={async () => {
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
                        await showErrorModalOnError(async () => await addGapAfter(selectedPacketStructureID()!, isField, index), 'Failed to add gap');
                    }}>Add Gap</button>
                </div>
            </div>
            {/* Packet structure component editor */}
            <div class="flex flex-col justify-between contentContainer">
                <Show when={selectedPacketStructureID() !== null} fallback={<h2 class="m-0 dark:text-white">No component selected</h2>}>
                    <div class="flex flex-col dark:text-white">
                        <Switch>
                            <Match when={selectedPacketComponentIndex() === null}>
                                <>Error, this should never display</>
                            </Match>
                            {/* Selected packet structure field editor */}
                            <Match when={selectedFieldData() !== null}>
                                <h2 class="m-0 font-bold">Field Information</h2>
                                <div class="flex flex-col gap-2">
                                    <div class="flex flex-col">
                                        <label for="fieldName">Name
                                            <input class="inputBox" type="text" value={selectedFieldData()!.name} id="fieldName"
                                                onInput={async e => await invokeApiSetter()(setFieldName, (e.target as HTMLInputElement).value)} 
                                                />
                                        </label>
                                    </div>
                                    <span>Offset in Packet: {selectedFieldData()!.offsetInPacket} byte{selectedFieldData()!.offsetInPacket == 1 ? "" : "s"}</span>
                                    <div class="flex flex-col">
                                        <label for="fieldType">Type
                                            <select class="inputBox" value={selectedFieldData()!.type} id="fieldType"
                                                onInput={async e => await invokeApiSetter()(setFieldType, ((e.target as HTMLSelectElement).value as PacketFieldType))}>
                                                <For each={Object.values(PacketFieldType).filter(k => isNaN(Number(k)))}>
                                                    {(fieldType) => <option value={fieldType}>{fieldType}</option>}
                                                </For>
                                            </select>
                                        </label>
                                    </div>
                                    <div class="flex flex-col">
                                        <label for="fieldMetadataType">Metadata Type
                                            <select class="inputBox" value={selectedFieldData()!.metadataType} id="fieldMetadataType"
                                                onInput={async e => await invokeApiSetter()(setFieldMetadataType, (e.target as HTMLSelectElement).value as PacketMetadataType)}>
                                                <For each={Object.values(PacketMetadataType).filter(k => isNaN(Number(k)))}>
                                                    {(metadataType) => <option value={metadataType}>{metadataType}</option>}
                                                </For>
                                            </select>
                                        </label>
                                    </div>
                                </div>
                            </Match>
                            {/* Selected packet structure delimiter editor */}
                            <Match when={selectedDelimiterData() !== null}>
                                <h2 class="m-0 font-bold text-xl">Delimiter Information</h2>
                                <div class="flex flex-col gap-2">
                                    <div class="flex flex-col">
                                        <label for="delimiterName" class="text-gray-700 dark:text-gray-400 text-sm">Name
                                        <input class="inputBox" type="text" value={selectedDelimiterData()!.name} id="delimiterName"
                                            onInput={async e => await invokeApiSetter()(setDelimiterName, (e.target as HTMLInputElement).value)} />
                                        </label>
                                    </div>
                                    <div>
                                        <label for="delimiterIdentifier">Identifier: 
                                        <input class="inputBox" type="text" value={selectedDelimiterData()!.identifier} id="delimiterIdentifier"
                                            onChange={async e => {
                                                const el = e.target as HTMLInputElement;
                                                el.value = el.value.replace(/[^\da-f]/g, '');
                                                await invokeApiSetter()(setDelimiterIdentifier, el.value);
                                            }} />
                                        </label>
                                    </div>
                                    <span>Offset in Packet: {selectedDelimiterData()!.offsetInPacket} byte{selectedDelimiterData()!.offsetInPacket == 1 ? "" : "s"}</span>
                                </div>
                            </Match>
                            {/* Selected packet structure gap editor */}
                            <Match when={selectedGapData() !== null}>
                                <h2 class="m-0">Gap Information</h2>
                                <div class="flex flex-col">
                                    <label for="gapSize">Size
                                    <input class="inputBox" type="number" value={selectedGapData()!.size} min={1} id="gapSize"
                                        onInput={async e => {
                                            const el = e.target as HTMLInputElement;
                                            el.value = el.value.replace(/[^\d]/g, '');
                                            if (+el.value < 1) {
                                                el.value = '1';
                                            }
                                            await showErrorModalOnError(
                                                async () => await setGapSize(selectedPacketStructureID()!, selectedGapData()!.offsetInPacket, +el.value),
                                                'Failed to change gap size'
                                            );
                                        }} />
                                    </label>
                                </div>
                            </Match>
                        </Switch>
                    </div>
                    <Switch>
                            <Match when={selectedPacketComponentIndex() === null}>
                                <>Error, this should never display</>
                            </Match>
                            {/* Selected packet structure field editor */}
                            <Match when={selectedFieldData() !== null}>
                                <button class="redButton relative bottom-0 pt-2 mt-5 m-0 dark:text-white hover:bg-gray-300 focus:outline-none focus:ring-4 
                                focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 bg-gray-100 text-black
                                dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700" 
                                    onClick={async () => await invokeApiSetter()(deletePacketStructureComponent, selectedPacketStructureComponent()!.type)}
                                >
                                    Delete {(selectedPacketStructureComponent()?.data as PacketField | PacketDelimiter)?.name ?? "Field"}
                                </button>
                            </Match>
                            {/* Selected packet structure delimiter editor */}
                            <Match when={selectedDelimiterData() !== null}>
                                <button class="redButton relative bottom-0 pt-2 mt-5 m-0 dark:text-white hover:bg-gray-300 focus:outline-none focus:ring-4 
                                focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 bg-gray-100 text-black
                                dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700" 
                                    onClick={async () => await invokeApiSetter()(deletePacketStructureComponent, selectedPacketStructureComponent()!.type)}
                                >
                                    Delete {(selectedPacketStructureComponent()?.data  as PacketField | PacketDelimiter)?.name ?? "Delimiter"}
                                </button>
                            </Match>
                            {/* Selected packet structure gap editor */}
                            <Match when={selectedGapData() !== null}>
                                <button class="redButton relative bottom-0 pt-2 mt-5 m-0 dark:text-white hover:bg-gray-300 focus:outline-none focus:ring-4 
                                focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 bg-gray-100 text-black
                                dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700" 
                                    onClick={async () => await setGapSize(selectedPacketStructureID()!, selectedGapData()!.offsetInPacket, 0)}
                                >
                                    Delete Gap
                                </button>
                            </Match>
                    </Switch>
                </Show>
            </div>
        </div>
    );
};

export default PacketEditor;