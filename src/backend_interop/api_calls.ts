import { invoke } from "@tauri-apps/api/tauri";
import { PacketComponentType, PacketFieldType, PacketMetadataType, RadioTestResult, PacketViewModel } from "./types";
/**
 * All direct function calls to the rust backend are sent through this file, 
 * returnErrorMessage is use for error proccessing on each function call.
 * The backend counterparts of each of the Functions are documented, so there will not be any documentation here
 */
/**
 * Calls the Rust backend function with the given name and arguments. If an error occurs, returns the error message as a string.
 * 
 * @param backend_function_name the name of the backend function defined in Rust to call
 * @param argument_values the object containing the arguments to pass to the function, if any
 * @returns the result of calling the Rust function with the given name or the error message as a string
 */
async function returnErrorMessage<T>(backend_function_name: string, argument_values?: any): Promise<string | T> {
    return await invoke<T>(backend_function_name, argument_values)
        .then(result => result)
        .catch(e => e as string);
}

export const setActivePort = async (portName: string): Promise<string | null> => await returnErrorMessage("set_active_port", { portName: portName });

let testWritePort: string | null;
let testReadPort: string | null;

export const setTestWritePort = async (portName: string): Promise<string | null> => {
    return await invoke("set_test_write_port", { portName: portName })
        .then(() => { 
            testWritePort = portName; 
            return null; 
        })
        .catch(e => e as string);
}

export const getTestWritePort = (): string | null => testWritePort;

export const setTestReadPort = async (portName: string): Promise<string | null> => {
    return await invoke("set_test_read_port", { portName: portName })
        .then(() => {
            testReadPort = portName;
            return null;
        })
        .catch(e => e as string);
}

export const getTestReadPort = (): string | null => testReadPort;

export const testRadios: () => Promise<string | RadioTestResult> = async () => await returnErrorMessage("test_radios");

export const setPacketName = async (packetStructureId: number, name: string) => await returnErrorMessage<void>("set_packet_name", { packetStructureId, name });

export const setFieldName = async (packetStructureId: number, fieldIndex: number, name: string) => await returnErrorMessage<void>("set_field_name", { packetStructureId, fieldIndex, name });

export const setFieldType = async (packetStructureId: number, fieldIndex: number, type: PacketFieldType) => await returnErrorMessage<void>("set_field_type", { packetStructureId, fieldIndex, type });

export const setFieldMetadataType = async (packetStructureId: number, fieldIndex: number, metadataType: PacketMetadataType) => await returnErrorMessage<void>("set_field_metadata_type", { packetStructureId, fieldIndex, metadataType });

export const setDelimiterName = async (packetStructureId: number, delimiterIndex: number, name: string) => await returnErrorMessage<void>("set_delimiter_name", { packetStructureId, delimiterIndex, name });

export const setDelimiterIdentifier = async (packetStructureId: number, delimiterIndex: number, identifier: string) => await returnErrorMessage<void>("set_delimiter_identifier", { packetStructureId, delimiterIndex, identifier });

export const setGapSize = async (packetStructureId: number, gapIndex: number, size: number) => await returnErrorMessage<void>("set_gap_size", { packetStructureId, gapIndex, size });

export const addField = async (packetStructureId: number) => await returnErrorMessage<void>("add_field", { packetStructureId: packetStructureId });

export const addDelimiter = async (packetStructureId: number) => await returnErrorMessage<void>("add_delimiter", { packetStructureId: packetStructureId });

export const addGapAfter = async (packetStructureId: number, isField: boolean, componentIndex: number) => await returnErrorMessage<void>("add_gap_after", { packetStructureId: packetStructureId, isField: isField, componentIndex: componentIndex });

export const deletePacketStructureComponent = async (packetStructureId: number, componentIndex: number, componentType: PacketComponentType) => await returnErrorMessage<void>("delete_packet_structure_component", { packetStructureId, componentIndex, componentType });

export const addPacket = async (view: PacketViewModel) => await returnErrorMessage("add_packet", { view });

export const registerEmptyPacketStructure = async () => await returnErrorMessage<void>("register_empty_packet_structure");

export const deletePacketStructure = async (packetStructureId: number) => await returnErrorMessage<void>('delete_packet_structure', { packetStructureId: packetStructureId });
