import { batch, Component } from "solid-js";
import FieldsScreen, { FlexviewObject, flexviewObjects, setDisplays, setFlexviewObjects } from "../components/DisplaySettingsScreen";
import logo from "../assets/logo.png";
import { useBackend } from "../backend_interop/BackendProvider";
import { useNavigate } from "@solidjs/router";
import { clearParsedPackets } from "../backend_interop/buffers";
import { save } from "@tauri-apps/api/dialog";
import { writeTextFile } from "@tauri-apps/api/fs";
import { displays } from "../components/DisplaySettingsScreen";
import { DisplayStruct } from "../core/display_registry";
import { store } from "../core/file_handling";

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
                { name: "SaveFlight", extensions: ["json"] }
            ]
        });

        if (selectedFilePath === null) {
            return;
        }
    };

    /**
     * Saves displays setup to a JSON file
     */
    const saveDisplaySetup = async () => {
        const selectedFilePath = await save({
            title: "Save Display Setup",
            filters: [
                { name: "SaveDisplaySetup", extensions: ["json"] }
            ]
        });

        if (!selectedFilePath) return;

        const displaySetupData = {
            "flexviewObjects": flexviewObjects as FlexviewObject[],
            "displays": displays as DisplayStruct[]
        }
        await writeTextFile(
            selectedFilePath, JSON.stringify(displaySetupData, null, 2));
    }

    /**
     * Removes all displays
     */
    const clearDisplaySetup = () => { 
        setFlexviewObjects([{
            type: 'layout',
            children: [],
            weights: [],
            direction: 'row'
        }]);
        setDisplays([]);

        store.set("display", displays);
        store.set("flexviewObjects", flexviewObjects);

        // Navigate home, since RecursiveFlexviewEditor isn't reactive  
        navigate("/");
    }


    return (
        <div class="flex flex-col flex-grow gap-4 rounded border dark:text-white">
            <div class="flex flex-grow h-0">
                {/* Views */}
                <div class="flex-grow grid grid-cols-1 p-2 gap-2 overflow-auto bg-gray-200 dark:bg-neutral-700" style={{ "width": "100%" }}>
                    <FieldsScreen />
                </div>
                {/* <div class="grid grid-cols-1 p-2 gap-2" style={{ "width": "100%" }}>
                    <PacketEditor/>
                </div> */}
            </div>

            {/* Actions bar */}
            <footer class="flex p-2 justify-between dark:bg-black-800 items-end">
                <div class="items-end flex-1">
                    {/* Homepage button */}
                    <button onClick={() => navigate("/")} class="flex items-center justify-center border-transparent bg-transparent">
                        <img src={logo} class="h-16" alt="Home" draggable={false} />
                    </button>
                    {/* <button type="button" >Dark</button> */}
                </div>
                <div class="items-center flex-3 flex g-2">
                    <p class="m-4">Packets Received: {parsedPacketCount()}</p>
                    <button
                        type="button"
                        onClick={clearParsedPackets}
                        class="text-black bg-gray-200 hover:bg-gray-400 focus:outline-none focus:ring-4 focus:ring-gray-300
                        font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 
                        dark:border-gray-700 dark:text-white"
                    >
                        Clear Graph
                    </button>
                    <button
                        type="button"
                        onClick={saveFlight}
                        class="text-dark bg-gray-200 hover:bg-gray-400 focus:outline-none focus:ring-4 focus:ring-gray-300
                        font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 
                        dark:border-gray-700 dark:text-white"
                    >
                        Save Flight
                    </button>
                    <button
                        type="button"
                        onClick={saveDisplaySetup}
                        class="text-dark bg-gray-200 hover:bg-gray-400 focus:outline-none focus:ring-4 focus:ring-gray-300
                            font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 
                            dark:border-gray-700 dark:text-white"
                    >
                        Save Display Setup
                    </button>
                    <button
                        type="button"
                        onClick={clearDisplaySetup}
                        class="text-dark bg-gray-200 hover:bg-gray-400 focus:outline-none focus:ring-4 focus:ring-gray-300
                            font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 
                            dark:border-gray-700 dark:text-white"
                    >
                        Clear Display Setup
                    </button>
                </div>
            </footer>
        </div>
    );
};

export default SettingsTab;