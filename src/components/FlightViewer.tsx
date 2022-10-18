import { Component, createSignal, For } from "solid-js";
import { setActivePort, setTestPort } from "../backend_interop/api_calls";
import { BackendInteropManagerContextValue, useBackendInteropManager } from "./BackendInteropManagerProvider";
import ThemeSwitcher from "./ThemeSwitcher";

const FlightViewer: Component = () => {
    const { availablePortNames }: BackendInteropManagerContextValue = useBackendInteropManager();

    const [selectedActivePort, setSelectedActivePort] = createSignal<string>();
    const [selectedTestPort, setSelectedTestPort] = createSignal<string>();

    return (
        <div class="flex flex-col p-4 gap-4 dark:bg-dark-700 h-full">
            <div class="flex flex-row-reverse">
                <ThemeSwitcher />
            </div>

            <div class="flex">
                <p>Active Port:</p>
                <input type="text" name="Serial Port" list="serialPorts" onInput={event => setSelectedActivePort((event.target as HTMLInputElement).value)} />
                <datalist id="serialPorts">
                    <For each={availablePortNames()}>
                        {(serialPort) => <option value={serialPort.name} /> }
                    </For>
                </datalist>
                <button onClick={() => setActivePort(selectedActivePort()!)} disabled={selectedActivePort() === undefined}>Connect</button>
            </div>

            <div class="flex">
                <p>Test Port:</p>
                <input type="text" name="Test Port" list="serialPorts" onInput={event => setSelectedTestPort((event.target as HTMLInputElement).value)} />
                <datalist id="serialPorts">
                    <For each={availablePortNames()}>
                        {(serialPort) => <option value={serialPort.name} /> }
                    </For>
                </datalist>
                <button onClick={() => setTestPort(selectedTestPort()!)} disabled={selectedTestPort() === undefined}>Connect</button>
            </div>
        </div>
    );
};

export default FlightViewer;