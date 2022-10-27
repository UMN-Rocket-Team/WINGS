import { invoke } from "@tauri-apps/api/tauri";
import { Packet, PacketFieldValue, PacketStructure, RefreshAndReadResult, RustPacket, RustRefreshAndReadResult } from "./types";

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

export const setTestPort = async (portName: string) => await invoke("set_test_port", { portName: portName });

export const writeTestPacketToTestPort = async () => await invoke("write_test_packet_to_test_port");

export const registerPacketStructure = async (packetStructure: PacketStructure) => 
    await invoke("register_packet_structure", { packetStructure: packetStructure });
