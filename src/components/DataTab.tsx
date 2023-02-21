import { Component, createEffect, createSignal, For } from "solid-js";
import { setActivePort } from "../backend_interop/api_calls";
import { useBackendInteropManager } from "./BackendInteropManagerProvider";
import GraphScreen from "./GraphScreen";

const DataTab: Component = () => {
    const { availablePortNames } = useBackendInteropManager();

    const [selectedPort, setSelectedPort] = createSignal<string | null>()

    createEffect(() => {
        if (selectedPort() != null) {
            setActivePort(selectedPort()!)
        }
    }, { defer: true });

    return (
        <>
            <div class="grid" style={{ "grid-template-columns": "1fr 1fr", "gap": "1rem", "grid-auto-rows": "1fr" }}>
                <GraphScreen />
                <GraphScreen />
                <GraphScreen />
                <GraphScreen />
            </div>
            <div class="flex gap-2">
                <label for="recievingRadioPortInput" class="dark:text-white">Reciving Radio Port:</label>
                <input type="text" name="Test Port" id="recievingRadioPortInput" list="testSerialPorts"
                    onInput={event => setSelectedPort((event.target as HTMLInputElement).value)} value={selectedPort() ?? ""} />
                <datalist id="testSerialPorts">
                    <For each={availablePortNames()}>
                        {(serialPort) => <option value={serialPort.name} />}
                    </For>
                </datalist>
            </div>
        </>
    );
};

export default DataTab;