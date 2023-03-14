import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from '@tauri-apps/api/dialog';
import { PacketComponentType, PacketFieldType, PacketMetadataType, RadioTestResult, RefreshAndReadResult } from "./types";
import { PacketViewModel } from "../backend_interop/types";
import { writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

export const importPacket = async () => {
    const selectedFilePaths = await open({title: 'Import Flight Data', multiple: true, filters: [{name: 'FlightData', extensions: ['json'] }] });
    if (Array.isArray(selectedFilePaths)) {
        // user selected multiple files
      } else if (selectedFilePaths === null) {
        // user cancelled the selections
      } else {
        // user selected a single file
      }
}

export const exportPacket = async (packetView: PacketViewModel[]) => {
  const selectedFilePath = await save({title: 'Export Flight Data', filters: [{name: 'FlightData', extensions: ['json'] }] });
  if (selectedFilePath != null)
  {
    await writeTextFile(
      {
        contents: JSON.stringify(packetView),
        path: selectedFilePath as string,
      },
    );
  }

}

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

export const setFieldName = async (packetStructureId: number, fieldIndex: number, name: string) => await invoke("set_field_name", { packetStructureId, fieldIndex, name });

export const setFieldType = async (packetStructureId: number, fieldIndex: number, type: PacketFieldType) => await invoke("set_field_type", { packetStructureId, fieldIndex, type });

export const setFieldMetadataType = async (packetStructureId: number, fieldIndex: number, metadataType: PacketMetadataType) => await invoke("set_field_metadata_type", { packetStructureId, fieldIndex, metadataType });

export const setDelimiterName = async (packetStructureId: number, delimiterIndex: number, name: string) => await invoke("set_delimiter_name", { packetStructureId, delimiterIndex, name });

export const setDelimiterIdentifier = async (packetStructureId: number, delimiterIndex: number, identifier: string) => await invoke("set_delimiter_identifier", { packetStructureId, delimiterIndex, identifier });

export const setGapSize = async (packetStructureId: number, gapIndex: number, size: number) => await invoke("set_gap_size", { packetStructureId, gapIndex, size });

export const addField = async (packetStructureId: number) => await invoke("add_field", { packetStructureId: packetStructureId });

export const addDelimiter = async (packetStructureId: number) => await invoke("add_delimiter", { packetStructureId: packetStructureId });

export const addGapAfter = async (packetStructureId: number, isField: boolean, componentIndex: number) => await invoke("add_gap_after", { packetStructureId: packetStructureId, isField: isField, componentIndex: componentIndex });

export const deletePacketStructureComponent = async (packetStructureId: number, componentIndex: number, componentType: PacketComponentType) => await invoke("delete_packet_structure_component", { packetStructureId, componentIndex, componentType });
