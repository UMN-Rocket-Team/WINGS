import {Component, For} from "solid-js";
import {SerialPortNames} from "../backend_interop/types";
import BroadcastModal from "./BroadcastModal";
import {useModal} from "./ModalProvider";
import FieldsPlayground from "./FieldsPlayground";

const DataTab: Component = () => {
    const { showModal } = useModal();
    // const connectionState = createSignal(true);

    // const { availablePortNames }: BackendInteropManagerContextValue = useBackendInteropManager();
    const sampleSerialPortNames: SerialPortNames[] = [
        {name: "Sample COM 1", manufacturer_name: "Sample Manufacturer 1", product_name: "Sample Product 1"},
        {name: "Sample COM 2", manufacturer_name: "Sample Manufacturer 2", product_name: "Sample Product 2"}
    ];

    return (
        <div class="flex flex-col flex-grow gap-4 dark:text-white">
            <FieldsPlayground></FieldsPlayground>

            {/*bottom bar*/}
            <footer class="flex p-2 bg-gray">
                <div class="flex w-2xl">
                    <p>[Back button icon]</p>
                    <p class="dark:text-white">Serial Port:</p>
                    <input list="dataSerialPorts" name="Serial Port :"/>
                    <datalist id="dataSerialPorts">
                        <For each={sampleSerialPortNames}>
                            {(serialPort) => <option value={serialPort.name} /> }
                        </For>
                    </datalist>

                    <button>Connect/Disconnect</button>
                </div>

                <div class="flex w-2xl">
                    <p>Packets Received: [# Packets Received]</p>
                </div>

                <div class="flex w-2xl">
                    <button onClick={() => showModal<{}, {}>(BroadcastModal, {})}>Broadcast</button>
                    <button>Save</button>
                    <button>Upload</button>
                </div>
            </footer>
        </div>
    );
};

export default DataTab;