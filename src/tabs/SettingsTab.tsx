import {Component, createSignal, For} from "solid-js";
import FieldsScreen from "../components/DisplaySettingsScreen";
import logo from "../assets/logo.png";
import {useBackend} from "../backend_interop/BackendProvider";
import {useNavigate} from "@solidjs/router";
import {Packet} from "../backend_interop/types";
import {clearParsedPackets, parsedPackets} from "../backend_interop/buffers";
import {writeFile} from "@tauri-apps/api/fs";
import {save} from "@tauri-apps/api/dialog";
import ErrorModal, {ErrorModalProps} from "../modals/ErrorModal";
import {useModal} from "../modals/ModalProvider";
import PacketEditor from "../components/PacketsEditor";

/**
 * Main Tab for hosting all groundstation settings
 */
const SettingsTab: Component = () => {
    const { availableDeviceNames: availablePortNames, PacketStructureViewModels, parsedPacketCount } = useBackend();
    const navigate = useNavigate();
    
    const saveFlight = async () => {
        const selectedFilePath = await save({
            title: "Save Flight",
            filters: [
                {name: "SaveFlight", extensions: ["json"]}
            ]
        });

        if (selectedFilePath === null) {
            return;
        }
    };

    return (
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
            <div class="flex flex-grow h-0">
                {/*Views*/}
                <div class="flex-grow grid grid-cols-1 p-2 gap-2" style={{ "width": "100%" }}>
                    <FieldsScreen/>
                </div>
                <div class="flex flex-grow grid grid-cols-1 p-2 gap-2" style={{ "width": "100%" }}>
                    <PacketEditor/>
                </div>
            </div>

            {/*Actions bar*/}
            <footer class="flex p-2 items-center justify-between drop-shadow-lightgray dark:drop-shadow-gray">
                <div class="flex items-center">
                    {/* Homepage button */}
                    <button onClick={() => navigate("/")} class="flex items-center justify-center border-transparent bg-transparent">
                        <img src={logo} height={25} alt="Home" draggable={false}></img>
                    </button>
                </div>
                <p class="m-0">Packets Received: {parsedPacketCount()}</p>
                <button onClick={clearParsedPackets}>Clear graph</button>
                <button onClick={saveFlight}>Save</button>
            </footer>
        </div>
    );
};

export default SettingsTab;