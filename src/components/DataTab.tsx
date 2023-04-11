import {Component, createEffect, createSignal, For} from "solid-js";
import FieldsScreenContainer from "./FieldsScreenContainer";
import logo from "../assets/logo.png";
import {useBackend} from "./BackendProvider";
import {setActivePort} from "../backend_interop/api_calls";
import {useNavigate} from "@solidjs/router";

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
    const navigate = useNavigate();
    const [selectedPort, setSelectedPort] = createSignal<string | null>();

    createEffect(() => {
        if (selectedPort() != null) {
            // Apply the change in selected port name to the backend
            setActivePort(selectedPort()!)
        }
    }, { defer: true });

    return (
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
            <FieldsScreenContainer packetViewModels={packetViewModels}></FieldsScreenContainer>

            {/*Actions bar*/}
            <footer class="flex p-2 items-center justify-between drop-shadow-lightgray dark:drop-shadow-gray">
                <div class="flex items-center">
                    {/* Homepage button */}
                    <button onClick={() => navigate("/")} class="flex items-center justify-center border-transparent bg-transparent">
                        <img src={logo} height={25} alt="Home"></img>
                    </button>
                    {/* Active serial port combobox */}
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

                <button>Save</button>
            </footer>
        </div>
    );
};

export default DataTab;