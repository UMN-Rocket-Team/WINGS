import {Component, createSignal, For} from "solid-js";
import {SerialPortNames} from "../backend_interop/types";
import BroadcastModal from "./BroadcastModal";
import {useModal} from "./ModalProvider";
import FieldsPlayground from "./FieldsPlayground";
import logo from "../assets/logo.png";
import UploadModal from "./UploadModal";

const DataTab: Component = () => {
    const { showModal } = useModal();
    // const connectionState = createSignal(true);

    const sampleSerialPortNames: SerialPortNames[] = [
        {name: "Sample COM 1", manufacturer_name: "Sample Manufacturer 1", product_name: "Sample Product 1"},
        {name: "Sample COM 2", manufacturer_name: "Sample Manufacturer 2", product_name: "Sample Product 2"}
    ];

    const availablePortNames = sampleSerialPortNames;
    const [packetsReceived, setPacketsReceived] = createSignal(0);

    return (
        <div class="flex flex-col flex-grow gap-4 dark:text-white">
            <FieldsPlayground></FieldsPlayground>

            {/*Actions bar*/}
            <footer class="flex p-2 gap-10 bg-gray">
                <div class="flex">
                    <a href="/">
                        <img src={logo} class={"h-5"} alt="Home"></img>
                    </a>
                    <p class="px-2 m-0">Serial Port :</p>
                    <input list="dataSerialPorts" name="Serial Port"/>
                    <datalist id="dataSerialPorts">
                        <For each={availablePortNames}>
                            {(serialPort) => <option value={serialPort.name} /> }
                        </For>
                    </datalist>

                    <button>Connect/Disconnect</button>
                </div>

                <p class="m-0">Packets Received: {packetsReceived}</p>

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