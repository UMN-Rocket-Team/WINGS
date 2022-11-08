import { Component, createSignal, For } from "solid-js";
import { setActivePort, setTestPort } from "../backend_interop/api_calls";
import { BackendInteropManagerContextValue, useBackendInteropManager } from "./BackendInteropManagerProvider";

const TestingTab: Component = () => {
    const { availablePortNames }: BackendInteropManagerContextValue = useBackendInteropManager();

    const [selectedActivePort, setSelectedActivePort] = createSignal<string>();
    const [selectedTestPort, setSelectedTestPort] = createSignal<string>();
    
    return (
        <>
            <div class="flex">
                <p>Active Port:</p>
                <input type="text" name="Serial Port" list="activeSerialPorts" onInput={event => setSelectedActivePort((event.target as HTMLInputElement).value)} />
                <datalist id="activeSerialPorts">
                    <For each={availablePortNames()}>
                        {(serialPort) => <option value={serialPort.name} /> }
                    </For>
                </datalist>
                <button onClick={() => setActivePort(selectedActivePort()!)} disabled={selectedActivePort() === undefined}>Connect</button>
            </div>

            <div class="flex">
                <p>Test Port:</p>
                <input type="text" name="Test Port" list="testSerialPorts" onInput={event => setSelectedTestPort((event.target as HTMLInputElement).value)} />
                <datalist id="testSerialPorts">
                    <For each={availablePortNames()}>
                        {(serialPort) => <option value={serialPort.name} /> }
                    </For>
                </datalist>
                <button onClick={() => setTestPort(selectedTestPort()!)} disabled={selectedTestPort() === undefined}>Connect</button>
            </div>
        </>
    );
};

export default TestingTab;