import { open, save } from '@tauri-apps/api/dialog';
import { PacketViewModel } from "../backend_interop/types";
import { writeTextFile,readTextFile } from '@tauri-apps/api/fs';
import {addPacket, debug} from "../backend_interop/api_calls"

export const importPacket = async () => {
  let importedPackets:PacketViewModel[] = [];

  const selectedFilePaths = await open({title: 'Import Flight Data', multiple: true, filters: [{name: 'FlightData', extensions: ['json'] }] });
  if (Array.isArray(selectedFilePaths)) {
      for(const path of selectedFilePaths){
        let contents = await readTextFile(path as string);
        let parsedContents: PacketViewModel = JSON.parse(contents)
        importedPackets.push(parsedContents)
      }
    } 
  else if(selectedFilePaths != null) {
    let contents = await readTextFile(selectedFilePaths as string);
    let parsedContents: PacketViewModel = JSON.parse(contents)
    importedPackets.push(parsedContents)
  }
  for (const packetView of importedPackets){
    addPacket(packetView)      
  }
}

export const exportPacket = async (packetView: PacketViewModel) => {
  const selectedFilePath = await save({title: 'Export Flight Data', filters: [{name: 'FlightData', extensions: ['json'] }] });
  if (selectedFilePath != null)
  {
    let data = JSON.stringify(packetView)
    let filePathString = selectedFilePath as string
    await writeTextFile({contents: data,path: filePathString,});
  }

}
