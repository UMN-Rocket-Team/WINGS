import { open, save } from '@tauri-apps/api/dialog';
import { PacketViewModel } from "../backend_interop/types";
import { writeTextFile,readTextFile } from '@tauri-apps/api/fs';

export const importPacket = async () => {
    const selectedFilePaths = await open({title: 'Import Flight Data', multiple: true, filters: [{name: 'FlightData', extensions: ['json'] }] });
    let importedPackets:string[] = [];
    if (Array.isArray(selectedFilePaths)) {
        for(const path of selectedFilePaths){
          const contents = await readTextFile(path as string);
          importedPackets.push(contents)
        }
      } 
      else if(selectedFilePaths != null) {
        const contents = await readTextFile(selectedFilePaths as string);
        importedPackets.push(contents)
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
