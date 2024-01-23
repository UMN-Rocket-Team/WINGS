import { open, save } from '@tauri-apps/api/dialog';
import { PacketViewModel, PacketComponentType } from "../backend_interop/types";
import { writeTextFile, readTextFile } from '@tauri-apps/api/fs';
import { addPacket } from "../backend_interop/api_calls";
import { Store } from"tauri-plugin-store-api";
/**
 * Exports the given PacketViewModel as a .json file via a dialouge window.
 * 
 * Creates File dialouge allowing the user to choose where to export the packetViewModel.
 * The function then writes to the selected file path.
 * 
 * @param {PacketViewModel} packetView the packet which will be contained in the experted file
 */
export const runExportPacketWindow = async (packetView: PacketViewModel) => {
    const selectedFilePath = await save({ title: 'Export Flight Data', defaultPath: packetView.name, filters: [{ name: 'FlightData', extensions: ['json'] }] });
    runExport(selectedFilePath, packetView);
}


const runExport = async (selectedFilePath: string | null, packetView: PacketViewModel) => {
    updatePersistentFilePaths(await exportToLocation(selectedFilePath, packetView));
};

/**
 * Writes a given packetViewModel to a selected File directory
 * 
 * @param selectedFilePath location where the save file will be created
 * @param packetView packetviewmodel to save to a file
 */
const exportToLocation = async (selectedFilePath: string | null, packetView: PacketViewModel) => {
    if (selectedFilePath != null) {
        let data: string = JSON.stringify(packetView);
        let filePathString: string = selectedFilePath as string;
        
        await writeTextFile({ contents: data, path: filePathString});
        return(filePathString);
    }
    return("");
}

/**
 * updates a queue list of the last 10 file paths that the function has been called with
 * this list is later accessed by a modal for future imports/exports
 * @param filePathString file path that needs to be saved as persistent data
 */
const updatePersistentFilePaths = async (filePathString: string) => {
    //adds new file directory to persistent data
    const store = new Store("persistent.dat");
    let prevSaves: String[] | null = await store.get("recentSaves");
    if( Array.isArray(prevSaves)){

        //limit length
        if (prevSaves.length >= 10){
            prevSaves.shift();

        }

        //get rid of repeates by filtering for only elements that arent equal to new element
        prevSaves = prevSaves.filter((value) => value != filePathString)

        prevSaves.push(filePathString);
        await store.set("recentSaves", prevSaves);

    } else {
        prevSaves = [filePathString];
        await store.set("recentSaves", prevSaves);
    }
    await store.save();
}

/**
 * Imports a set of packets selected by the user via a dialouge window.
 * 
 * Creates a file dialouge box, allowing user to select multiple .json packet files. returns the file directories of said packets
 */
export const runImportPacketWindow = async () => {
    const selectedFilePaths = await open({ title: 'Import Flight Data', multiple: true, filters: [{ name: 'FlightData', extensions: ['json'] }] });
    return selectedFilePaths;
}

export const importPacketsfromDirectories = async (filePaths: string | string[] | null)=>{
    const filePackets = await openPackets(filePaths);
    for (const packetView of filePackets) {
        addPacket(packetView);
    }
}
/**
 * Imports from selected file path/paths.
 * 
 * Converts json files to packetViewModels using readPathAsPacket().
 * 
 * @param selectedFilePaths an array of strigns containing file directories
 * 
 * @return {PacketViewModel} the PacketViewModels stored at the given directories
 */
export const openPackets = async (selectedFilePaths: string | string[] | null) => {
    let openedPackets: PacketViewModel[] = [];
    if (Array.isArray(selectedFilePaths)) {
        for (const path of selectedFilePaths) {
            openedPackets.push(await readPathAsPacket(path));
        }
    }
    else if (selectedFilePaths != null) {
        openedPackets.push(await readPathAsPacket(selectedFilePaths));
    }
    return openedPackets;
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

//in-source unit testing, mocks all I/O functions with fake variables
if (import.meta.vitest) {
    const { beforeEach, afterEach, describe, expect, it, vi } = import.meta.vitest
    const testDirectoryContents = '{"id":0,"name":"testPacketView","components":[{"type":"Delimiter","data":{"index":0,"name":"testDelimiter","identifier":"1D3NT1TY","offsetInPacket":0}}]}'
    const testPacketView = {id: 0, name: "testPacketView", components: [{type: PacketComponentType.Delimiter,data:{index: 0,name: "testDelimiter", identifier: "1D3NT1TY", offsetInPacket: 0}}]};
    const testDirectory = "fakeDirectory"

    describe("Describe", async () => {
        beforeEach(async ()=> {
            //replaces functions that read files
            vi.mock('@tauri-apps/api/fs',() => ({
                writeTextFile: vi.fn(),
                readTextFile: vi.fn().mockResolvedValue('{"id":0,"name":"testPacketView","components":[{"type":"Delimiter","data":{"index":0,"name":"testDelimiter","identifier":"1D3NT1TY","offsetInPacket":0}}]}')
            }))
            //replaces functions that require user input
            vi.mock('@tauri-apps/api/dialog',() => ({
                save: vi.fn().mockResolvedValue("fakeDirectory"),
                open: vi.fn().mockResolvedValue("fakeDirectory")
            }))
            //replaces functions that reachout to backend
            vi.mock('../backend_interop/api_calls',() => ({
                addPacket: vi.fn()
            }))
            
        })
        
        afterEach(()=>{
            vi.restoreAllMocks();
        })

        it('packet_file_I/O', async () => {
            await importPacketsfromDirectories(testDirectory);
            expect(addPacket).toBeCalledWith(testPacketView);
            expect(addPacket).toHaveBeenCalledTimes(1);

            await exportToLocation(testDirectory,testPacketView);
            expect(writeTextFile).toBeCalledWith({contents: testDirectoryContents, path: testDirectory});
            expect(writeTextFile).toHaveBeenCalledTimes(1);
        })
    })
}