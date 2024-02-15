import {Component, createSignal, For} from "solid-js";
import FieldsScreen from "./FieldsScreen";
import logo from "../assets/logo.png";
import {useBackend} from "./BackendProvider";
import {setActivePort} from "../backend_interop/api_calls";
import {useNavigate} from "@solidjs/router";
import {Packet} from "../backend_interop/types";
import {parsedPackets} from "../backend_interop/buffers";
import {writeFile} from "@tauri-apps/api/fs";
import {save} from "@tauri-apps/api/dialog";
import ErrorModal, {ErrorModalProps} from "./ErrorModal";
import {useModal} from "./ModalProvider";

const [selectedPort, setSelectedPort] = createSignal<string | null>();

/**
 * A component that allows the user to:
 *  - Customize four screens with different packet fields
 *  - Display screens as a modal to view the data received for the fields on the screen
 *  - Clear screens
 *  - Return to the homepage
 *  - Select the serial port to read data from
 *  - Read the number of parsed packets
 *  - Save flight data
 */
const DataTab: Component = () => {
    const { availablePortNames, packetViewModels, parsedPacketCount } = useBackend();
    const { showModal } = useModal();
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

        const parsedPacketsArray: Packet[] = Object.entries(parsedPackets).map(([structureId, packetDataArray]) =>
            packetDataArray.map((packetData) => (
                {structureId: +structureId, fieldData: packetData.fieldData, timestamp: packetData.timestamp}
            ))
        ).flat();

        writeFile(selectedFilePath as string, JSON.stringify({parsedPacketsArray, packetViewModels}))
            .catch((err) => showModal<ErrorModalProps, {}>(ErrorModal, {
                error: "Failed to Save Flight File",
                description: err
            }));
    };

    async function applyNewSelectedPort(newSelectedPort: string) {
        // Apply the change in selected port name to the backend
        try {
            setSelectedPort(newSelectedPort);
            await setActivePort(newSelectedPort);
        } catch (error) {
            showModal(ErrorModal, {error: 'Failed to set the active serial port', description: `${error}`});
        }
    }

    return (
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
            <div class="flex flex-grow h-0">
                {/*Views*/}
                <div class="grid grid-cols-1 p-2 gap-2" style={{ "width": "100%" }}>
                    <FieldsScreen/>
                </div>
            </div>

            {/*Actions bar*/}
            <footer class="flex p-2 items-center justify-between drop-shadow-lightgray dark:drop-shadow-gray">
                <div class="flex items-center">
                    {/* Homepage button */}
                    <button onClick={() => navigate("/")}
                            class="flex items-center justify-center border-transparent bg-transparent">
                        <img src={logo} height={25} alt="Home" draggable={false}></img>
                    </button>
                    {/* Active serial port combobox */}
                    <label for="serialPortInput" class="px-2 m-0">Serial Port:</label>
                    <input name="Serial Port" id="serialPortInput" class="w-50"
                        list="dataSerialPorts" value={selectedPort() ?? ""}
                        onChange={event => applyNewSelectedPort((event.target as HTMLInputElement).value)} />
                    <datalist id="dataSerialPorts">
                        <For each={availablePortNames()}>
                            {(serialPort) => <option value={serialPort.name}/>}
                        </For>
                    </datalist>
                </div>

                <p class="m-0">Packets Received: {parsedPacketCount()}</p>

                <button onClick={saveFlight}>Save</button>
            </footer>
        </div>
    );
};

export default DataTab;