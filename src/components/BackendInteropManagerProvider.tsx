import { Accessor, createContext, createSignal, onCleanup, onMount, ParentComponent, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { refreshAvailablePortsAndReadActivePort } from "../backend_interop/api_calls";
import { pushUnparsedPackets as pushParsedPackets } from "../backend_interop/buffers";
import { PacketComponentType, PacketData, PacketGap, PacketViewModel, RustPacketDelimiter, RustPacketField, RustPacketViewModel, SerialPortNames } from "../backend_interop/types";
import { emit, listen } from "@tauri-apps/api/event";
import { toPacketFieldType } from "../core/packet_field_type";

/**
 * The number of milliseconds to wait between refreshing the available serial ports and reading from the active port.
 */
const REFRESH_AND_READ_INTERVAL_MILLISECONDS = 1000;

export type BackendInteropManagerContextValue = {
    availablePortNames: Accessor<SerialPortNames[]>,
    newParsedPackets: Accessor<Record<number, PacketData[]> | undefined>,
    packetViewModels: PacketViewModel[],
};

const BackendInteropManagerContext = createContext<BackendInteropManagerContextValue>({
    availablePortNames: (): SerialPortNames[] => [],
    newParsedPackets: (): Record<number, PacketData[]> | undefined => undefined,
    packetViewModels: [],
});

export const BackendInteropManagerProvider: ParentComponent = (props) => {
    const [availablePortNames, setAvailablePortNames] = createSignal<SerialPortNames[]>([]);
    const [newParsedPackets, setNewParsedPackets] = createSignal<Record<number, PacketData[]>>();
    const [packetViewModels, setPacketViewModels] = createStore<PacketViewModel[]>([]);

    let refreshIntervalId: number;
    let unlistenFunction: () => void;

    onMount(async () => {
        refreshIntervalId = window.setInterval(async (): Promise<void> => {
            const result = await refreshAvailablePortsAndReadActivePort();

            if (result.new_available_port_names) {
                setAvailablePortNames(result.new_available_port_names);
            }
            if (result.parsed_packets) {
                setNewParsedPackets(pushParsedPackets(result.parsed_packets));
            }
        }, REFRESH_AND_READ_INTERVAL_MILLISECONDS);

        unlistenFunction = await listen<RustPacketViewModel[]>("packet-structures-update", event => {
            console.log(event);
            const newPacketViewModels = event.payload.map<PacketViewModel>(packetViewModel => {
                return ({
                    id: packetViewModel.id,
                    name: packetViewModel.name,
                    components: packetViewModel.components.map(component => {
                        if (Object.hasOwn(component, "Field")) {
                            const rustField = (component as any).Field as RustPacketField;                            

                            return {
                                type: PacketComponentType.Field,
                                data: {
                                    index: rustField.index,
                                    name: rustField.name,
                                    type: toPacketFieldType(rustField.type),
                                    offsetInPacket: rustField.offset_in_packet,
                                    metadataType: rustField.metadata_type
                                },
                            };
                        } else if (Object.hasOwn(component, "Delimiter")) {
                            const rustDelimiter = (component as any).Delimiter as RustPacketDelimiter;

                            return {
                                type: PacketComponentType.Delimiter,
                                data: {
                                    index: rustDelimiter.index,
                                    name: rustDelimiter.name,
                                    offsetInPacket: rustDelimiter.offset_in_packet,
                                    identifier: rustDelimiter.identifier.map(identifier => identifier.toString(16)).join(''),
                                },
                            };
                        } else {
                            return {
                                type: PacketComponentType.Gap,
                                data: (component as any).Gap as PacketGap
                            }
                        }
                    }),
                });
            });

            for (const packetViewModel of newPacketViewModels) {
                if (packetViewModels.some(oldPacketViewModel => oldPacketViewModel.id === packetViewModel.id)) {
                    setPacketViewModels(
                        oldPacketViewModel => oldPacketViewModel.id === packetViewModel.id,
                        packetViewModel
                    );
                } else {
                    setPacketViewModels(
                        packetViewModels.length,
                        packetViewModel
                    );
                }
            }
        });

        await emit("initialized");
    });

    onCleanup((): void => {
        clearInterval(refreshIntervalId);
        unlistenFunction();
    });

    const context = { availablePortNames: availablePortNames, newParsedPackets: newParsedPackets, packetViewModels: packetViewModels };

    return (
        <BackendInteropManagerContext.Provider value={context}>
            {props.children}
        </BackendInteropManagerContext.Provider>
    );
};

export const useBackendInteropManager = (): BackendInteropManagerContextValue => useContext(BackendInteropManagerContext);
