import {Component, createEffect, createSignal, For} from "solid-js";
import BroadcastModal from "./BroadcastModal";
import {useModal} from "./ModalProvider";
import FieldsPlayground from "./FieldsPlayground";
import logo from "../assets/logo.png";
import UploadModal from "./UploadModal";
import {useBackendInteropManager} from "./BackendInteropManagerProvider";
import {setActivePort} from "../backend_interop/api_calls";
import {useNavigate} from "@solidjs/router";
import {PacketComponentType, PacketFieldType, PacketMetadataType, PacketViewModel} from "../backend_interop/types";

// const samplePacketViewModels: PacketViewModel[] = [
//     {
//         id: 69,
//         name: "Packet 1 Name",
//         components: [
//             { type: PacketComponentType.Field, data: { index: 0, name: "Field 1", type: PacketFieldType.SignedInteger, offsetInPacket: 0, metadataType: PacketMetadataType.None } },
//             { type: PacketComponentType.Field, data: { index: 1, name: "Field 2", type: PacketFieldType.SignedInteger, offsetInPacket: 0, metadataType: PacketMetadataType.Timestamp } }
//         ]
//     },
//     {
//         id: 70,
//         name: "Packet 2 Name",
//         components: [
//             { type: PacketComponentType.Field, data: { index: 0, name: "Field 1", type: PacketFieldType.SignedInteger, offsetInPacket: 0, metadataType: PacketMetadataType.None } }
//         ]
//     }
// ];

const DataTab: Component = () => {
    const { showModal } = useModal();
    const { availablePortNames, packetViewModels, parsedPacketCount } = useBackendInteropManager();
    const navigate = useNavigate();
    const [selectedPort, setSelectedPort] = createSignal<string | null>();

    createEffect(() => {
        if (selectedPort() != null) {
            setActivePort(selectedPort()!)
        }
    }, { defer: true });

    return (
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
            <FieldsPlayground packetViewModels={packetViewModels}></FieldsPlayground>

            {/*Actions bar*/}
            <footer class="flex p-2 items-center justify-between drop-shadow-lightgray dark:drop-shadow-gray">
                <div class="flex items-center">
                    <button onClick={() => navigate("/")} class="flex items-center justify-center border-transparent bg-transparent">
                        <img src={logo} height={25} alt="Home"></img>
                    </button>
                    <label for="serialPortInput" class="px-2 m-0">Serial Port:</label>
                    <input list="dataSerialPorts" name="Serial Port" id="serialPortInput"
                        onInput={event => setSelectedPort((event.target as HTMLInputElement).value)} value={selectedPort() ?? ""}/>
                    <datalist id="dataSerialPorts">
                        <For each={availablePortNames()}>
                            {(serialPort) => <option value={serialPort.name} /> }
                        </For>
                    </datalist>
                </div>

                <p class="m-0">Packets Received: {parsedPacketCount()}</p>

                <div class="flex gap-1">
                    <button onClick={() => showModal<{}, {}>(BroadcastModal, {})}>Broadcast</button>
                    <button>Save</button>
                    <button onClick={() => showModal<{}, {}>(UploadModal, {})}>Upload</button>
                </div>
            </footer>
        </div>
    );
};

export default DataTab;