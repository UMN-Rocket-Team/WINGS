import { invoke } from "@tauri-apps/api/tauri";
import { PacketComponentType, PacketFieldType, PacketMetadataType, PacketViewModel } from "./types";

/*
 * All direct function calls to the rust backend are sent through this file, 
 * The backend counterparts of each of the functions are documented, so there will not be any documentation here
 */

export const setActivePort = async (portName: string) => await invoke<void>("set_active_port", { portName: portName });

export const startRadioTest = async (sendPort: string, receivePort: string, sendInterval: number) => await invoke<void>("start_radio_test", { sendPort, sendInterval, receivePort });

export const stopRadioTest = async () => await invoke<void>("stop_radio_test");

export const setPacketName = async (packetStructureId: number, name: string) => await invoke<void>("set_packet_name", { packetStructureId, name });

export const setFieldName = async (packetStructureId: number, fieldIndex: number, name: string) => await invoke<void>("set_field_name", { packetStructureId, fieldIndex, name });

export const setFieldType = async (packetStructureId: number, fieldIndex: number, type: PacketFieldType) => await invoke<void>("set_field_type", { packetStructureId, fieldIndex, type });

export const setFieldMetadataType = async (packetStructureId: number, fieldIndex: number, metadataType: PacketMetadataType) => await invoke<void>("set_field_metadata_type", { packetStructureId, fieldIndex, metadataType });

export const setDelimiterName = async (packetStructureId: number, delimiterIndex: number, name: string) => await invoke<void>("set_delimiter_name", { packetStructureId, delimiterIndex, name });

export const setDelimiterIdentifier = async (packetStructureId: number, delimiterIndex: number, identifier: string) => await invoke("set_delimiter_identifier", { packetStructureId, delimiterIndex, identifier });

export const setGapSize = async (packetStructureId: number, gapIndex: number, size: number) => await invoke<void>("set_gap_size", { packetStructureId, gapIndex, size });

export const addField = async (packetStructureId: number) => await invoke<void>("add_field", { packetStructureId: packetStructureId });

export const addDelimiter = async (packetStructureId: number) => await invoke<void>("add_delimiter", { packetStructureId: packetStructureId });

export const addGapAfter = async (packetStructureId: number, isField: boolean, componentIndex: number) => await invoke<void>("add_gap_after", { packetStructureId: packetStructureId, isField: isField, componentIndex: componentIndex });

export const deletePacketStructureComponent = async (packetStructureId: number, componentIndex: number, componentType: PacketComponentType) => await invoke<void>("delete_packet_structure_component", { packetStructureId, componentIndex, componentType });

export const addPacket = async (view: PacketViewModel) => await invoke<void>("add_packet", { view });

export const registerEmptyPacketStructure = async () => await invoke<void>("register_empty_packet_structure");

export const deletePacketStructure = async (packetStructureId: number) => await invoke<void>('delete_packet_structure', { packetStructureId: packetStructureId });
