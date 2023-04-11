import { open, save } from '@tauri-apps/api/dialog';
import { PacketViewModel } from "../backend_interop/types";
import { writeTextFile, readTextFile } from '@tauri-apps/api/fs';
import { addPacket } from "../backend_interop/api_calls"

export const importPacket = async () => {
    let importedPackets: PacketViewModel[] = [];

    const selectedFilePaths = await open({ title: 'Import Flight Data', multiple: true, filters: [{ name: 'FlightData', extensions: ['json'] }] });
    if (Array.isArray(selectedFilePaths)) {
        for (const path of selectedFilePaths) {
            importedPackets.push(await pathToPacketViewModel(path));
        }
    }
    else if (selectedFilePaths != null) {
        importedPackets.push(await pathToPacketViewModel(selectedFilePaths));
    }
    for (const packetView of importedPackets) {
        addPacket(packetView);
    }
}

const pathToPacketViewModel = async (path: string) => {
    let contents = await readTextFile(path as string);
    let parsedContents: PacketViewModel = JSON.parse(contents);
    return parsedContents;
}

export const exportPacket = async (packetView: PacketViewModel) => {
    const selectedFilePath = await save({ title: 'Export Flight Data', defaultPath: packetView.name, filters: [{ name: 'FlightData', extensions: ['json'] }] });
    if (selectedFilePath != null) {
        let data = JSON.stringify(packetView);
        let filePathString = selectedFilePath as string;
        await writeTextFile({ contents: data, path: filePathString, });
    }
}