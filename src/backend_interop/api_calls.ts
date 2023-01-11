import { invoke } from "@tauri-apps/api/tauri";
import { Packet, PacketFieldType, PacketFieldValue, PacketStructure, RadioTestResult, RefreshAndReadResult, RustPacket, RustRefreshAndReadResult } from "./types";

export const refreshAvailablePortsAndReadActivePort = async (): Promise<RefreshAndReadResult> => {
    const { new_available_port_names, parsed_packets } = await invoke<RustRefreshAndReadResult>("refresh_available_ports_and_read_active_port");

    if (parsed_packets === null) {
        return { new_available_port_names, parsed_packets };
    }

    // TODO: can PacketFieldValues be serialized in a more efficent manner in Rust so they don't have to be simplified here?
    const simplifiedParsedPackets = parsed_packets.map((rustPacket: RustPacket): Packet => ({
        fieldData: rustPacket.field_data.map((fieldValue: PacketFieldValue): number => Object.entries(fieldValue)[0][1] as number), 
        structureId: rustPacket.structure_id, 
        timestamp: rustPacket.timestamp
    }));

    return { new_available_port_names, parsed_packets: simplifiedParsedPackets };
};

export const setActivePort = async (portName: string) => await invoke("set_active_port", { portName: portName });

let testWritePort: string | null;
let testReadPort: string | null;

export const setTestWritePort = async (portName: string) => {
    await invoke("set_test_write_port", { portName: portName });
    testWritePort = portName;
}

export const getTestWritePort = () => testWritePort;

export const setTestReadPort = async (portName: string) => {
    await invoke("set_test_read_port", { portName: portName });
    testReadPort = portName;
}

export const getTestReadPort = () => testReadPort;

export const testRadios: () => Promise<RadioTestResult> = async () => await invoke("test_radios");

export const registerPacketStructure = async (packetStructure: PacketStructure) => 
    await invoke("register_packet_structure", { packetStructure: packetStructure });

export const setFieldName = async (packetStructureId: number, fieldIndex: number, name: string) => await invoke("set_field_name", { packetStructureId, fieldIndex, name });

export const setFieldType = async (packetStructureId: number, fieldIndex: number, type: PacketFieldType) => await invoke("set_field_type", { packetStructureId, fieldIndex, type });