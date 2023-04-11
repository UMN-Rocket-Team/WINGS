import { open, save } from '@tauri-apps/api/dialog';
import { PacketViewModel } from "../backend_interop/types";
import { writeTextFile, readTextFile } from '@tauri-apps/api/fs';
import { addPacket } from "../backend_interop/api_calls"

/**
 * Exports the given PacketViewModel as a .json file.
 * 
 * Creates File dialouge allowing the user to choose where to export the packetViewModel
 * 
 * @param {PacketViewModel} packetView the packet which will be contained in the experted file
 */
export const exportPacket = async (packetView: PacketViewModel) => {
    const selectedFilePath = await save({ title: 'Export Flight Data', defaultPath: packetView.name, filters: [{ name: 'FlightData', extensions: ['json'] }] });
    if (selectedFilePath != null) {
        let data: string = JSON.stringify(packetView);
        let filePathString: string = selectedFilePath as string;
        await writeTextFile({ contents: data, path: filePathString, });
    }
}

/**
 * Imports a set of packets selected by the user.
 * 
 * Creates a file dialouge box, allowing user to select multiple .json packet files.
 * The function then adds all selected packets to the internal packet structure using the addPacket rust function.
 */
export const importPacket = async () => {
    let importedPackets: PacketViewModel[] = [];
    const selectedFilePaths = await open({ title: 'Import Flight Data', multiple: true, filters: [{ name: 'FlightData', extensions: ['json'] }] });

    if (Array.isArray(selectedFilePaths)) {
        for (const path of selectedFilePaths) {
            importedPackets.push(await readPathAsPacket(path));
        }
    }
    else if (selectedFilePaths != null) {
        importedPackets.push(await readPathAsPacket(selectedFilePaths));
    }
    for (const packetView of importedPackets) {
        addPacket(packetView);
    }
}

/**
 * Returns the contents of a .json packet file at given file path.
 * 
 * @param path string containing the file path of a .json packet file
 * 
 * @return {PacketViewModel} the PacketViewModel stored at the given file path
 */
const readPathAsPacket = async (path: string) => {
    let contents: string = await readTextFile(path as string);
    let parsedContents: PacketViewModel = JSON.parse(contents);
    return parsedContents;
}