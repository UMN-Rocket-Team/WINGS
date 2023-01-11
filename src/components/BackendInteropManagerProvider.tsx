import { Accessor, createContext, createSignal, onCleanup, onMount, ParentComponent, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { refreshAvailablePortsAndReadActivePort, setFieldName, setFieldType } from "../backend_interop/api_calls";
import { pushUnparsedPackets as pushParsedPackets } from "../backend_interop/buffers";
import { PacketData, PacketFieldType, PacketMetadataType, PacketStructure, SerialPortNames } from "../backend_interop/types";

/**
 * The number of milliseconds to wait between refreshing the available serial ports and reading from the active port.
 */
const REFRESH_AND_READ_INTERVAL_MILLISECONDS = 1000;

export type BackendInteropManagerContextValue = {
    availablePortNames: Accessor<SerialPortNames[]>,
    newParsedPackets: Accessor<Record<number, PacketData[]> | undefined>,
    packetStructures: PacketStructure[],
    setFieldName: (packetStructureId: number, fieldIndex: number, name: string) => void,
    setFieldType: (packetStructureId: number, fieldIndex: number, type: PacketFieldType) => void,
};

const BackendInteropManagerContext = createContext<BackendInteropManagerContextValue>({
    availablePortNames: (): SerialPortNames[] => [],
    newParsedPackets: (): Record<number, PacketData[]> | undefined => undefined,
    packetStructures: [],
    setFieldName: () => { },
    setFieldType: () => { },
});

export const BackendInteropManagerProvider: ParentComponent = (props) => {
    const [availablePortNames, setAvailablePortNames] = createSignal<SerialPortNames[]>([]);
    const [newParsedPackets, setNewParsedPackets] = createSignal<Record<number, PacketData[]>>();
    const [packetStructures, setPacketStructures] = createStore<PacketStructure[]>([
        { id: 0, name: "Packet 0", fields: [{ metadataType: PacketMetadataType.None, name: "Field 0", offsetInPacket: 0, type: PacketFieldType.UnsignedInteger }, { metadataType: PacketMetadataType.None, name: "Field 1", offsetInPacket: 4, type: PacketFieldType.Double }], delimiters: [{ identifier: new Uint8Array([0xFF]), name: "Delimiter 0", offsetInPacket: 8 }] },
        { id: 1, name: "Packet 1", fields: [{ metadataType: PacketMetadataType.Timestamp, name: "Timestamp", offsetInPacket: 0, type: PacketFieldType.UnsignedLong }, { metadataType: PacketMetadataType.None, name: "Field 2", offsetInPacket: 8, type: PacketFieldType.SignedShort }], delimiters: [{ identifier: new Uint8Array([0xFF]), name: "Delimiter 2", offsetInPacket: 20 }] }
    ]);

    let refreshIntervalId: number;

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
    });

    onCleanup((): void => clearInterval(refreshIntervalId));

    const setFieldNameWrapper = (packetStructureId: number, fieldIndex: number, name: string) => {
        setFieldName(packetStructureId, fieldIndex, name);
        setPacketStructures(
            packetStructure => packetStructure.id === packetStructureId,
            "fields",
            fieldIndex,
            "name",
            name
        );
    };

    const setFieldTypeWrapper = (packetStructureId: number, fieldIndex: number, type: PacketFieldType) => {
        setFieldType(packetStructureId, fieldIndex, type.replaceAll(" ", "") as PacketFieldType);
        setPacketStructures(
            packetStructure => packetStructure.id === packetStructureId,
            "fields",
            fieldIndex,
            "type",
            type
        );
        console.log(packetStructures[packetStructureId].fields[fieldIndex].type);
    };

    const context = { availablePortNames: availablePortNames, newParsedPackets: newParsedPackets, packetStructures: packetStructures, setFieldName: setFieldNameWrapper, setFieldType: setFieldTypeWrapper };

    return (
        <BackendInteropManagerContext.Provider value={context}>
            {props.children}
        </BackendInteropManagerContext.Provider>
    );
};

export const useBackendInteropManager = (): BackendInteropManagerContextValue => useContext(BackendInteropManagerContext);
