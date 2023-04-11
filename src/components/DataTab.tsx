import {Component, createEffect, createSignal, For} from "solid-js";
import FieldsPlayground from "./FieldsPlayground";
import logo from "../assets/logo.png";
import {useBackendInteropManager} from "./BackendInteropManagerProvider";
import {setActivePort} from "../backend_interop/api_calls";
import {useNavigate} from "@solidjs/router";
import {
    Packet,
    PacketComponentType,
    PacketFieldType,
    PacketMetadataType,
    PacketViewModel
} from "../backend_interop/types";
import {parsedPackets} from "../backend_interop/buffers";
import {writeFile} from "@tauri-apps/api/fs";
import {save} from "@tauri-apps/api/dialog";

// const samplePacketViewModels: PacketViewModel[] = [
//     {
//         id: 69,
//         name: "Packet 1 Name",
//         components: [
//             {type: PacketComponentType.Field,
//                 data: {
//                     index: 0,
//                     name: "Field 1",
//                     type: PacketFieldType.SignedInteger,
//                     offsetInPacket: 0,
//                     metadataType: PacketMetadataType.None
//                 }
//             },
//             {type: PacketComponentType.Field,
//                 data: {
//                     index: 1,
//                     name: "Field 2",
//                     type: PacketFieldType.SignedInteger,
//                     offsetInPacket: 0,
//                     metadataType: PacketMetadataType.Timestamp
//                 }
//             }
//         ]
//     },
//     {
//         id: 70,
//         name: "Packet 2 Name",
//         components: [
//             {type: PacketComponentType.Field,
//                 data: {
//                     index: 0,
//                     name: "Field 1",
//                     type: PacketFieldType.SignedInteger,
//                     offsetInPacket: 0,
//                     metadataType: PacketMetadataType.None
//                 }
//             }
//         ]
//     }
// ];

const DataTab: Component = () => {
    const {availablePortNames, packetViewModels, parsedPacketCount} = useBackendInteropManager();
    const navigate = useNavigate();
    const [selectedPort, setSelectedPort] = createSignal<string | null>();

    const saveState = async () => {
        const selectedFilePath = await save({
            title: "Save State",
            filters: [
                {name: ".*", extensions: ["json"]}
            ]
        });

        if (selectedFilePath === null) {
            return;
        }

        const parsedPacketsArray: Packet[] = Object.entries(parsedPackets).map(([structureId, packetDataArray]) =>
            packetDataArray.map((packetData) => (
                {structureId: parseInt(structureId), fieldData: packetData.fieldData, timestamp: packetData.timestamp}
            ))
        ).flat();

        writeFile(selectedFilePath as string, JSON.stringify(parsedPacketsArray))
            .then(() => console.log("Saved state to " + selectedFilePath + "."))
            .catch((err) => console.error(err))
    };

    createEffect(() => {
        if (selectedPort() != null) {
            setActivePort(selectedPort()!)
        }
    }, {defer: true});

    return (
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
            <FieldsPlayground packetViewModels={packetViewModels}></FieldsPlayground>

            {/*Actions bar*/}
            <footer class="flex p-2 items-center justify-between drop-shadow-lightgray dark:drop-shadow-gray">
                <div class="flex items-center">
                    <button onClick={() => navigate("/")}
                            class="flex items-center justify-center border-transparent bg-transparent">
                        <img src={logo} height={25} alt="Home"></img>
                    </button>
                    <label for="serialPortInput" class="px-2 m-0">Serial Port:</label>
                    <input list="dataSerialPorts" name="Serial Port" id="serialPortInput"
                           onInput={event => setSelectedPort((event.target as HTMLInputElement).value)}
                           value={selectedPort() ?? ""}/>
                    <datalist id="dataSerialPorts">
                        <For each={availablePortNames()}>
                            {(serialPort) => <option value={serialPort.name}/>}
                        </For>
                    </datalist>
                </div>

                <p class="m-0">Packets Received: {parsedPacketCount()}</p>

                <div class="flex gap-1">
                    {/*<button onClick={() => showModal<{}, {}>(BroadcastModal, {})}>Broadcast</button>*/}
                    <button onClick={saveState}>Save</button>
                    {/*<button onClick={() => showModal<{}, {}>(UploadModal, {})}>Upload</button>*/}
                </div>
            </footer>
        </div>
    );
};

export default DataTab;