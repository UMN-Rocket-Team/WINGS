import { batch, Component, createEffect, createSignal, For, onCleanup, untrack } from "solid-js";
import { getTestReadPort, getTestWritePort, setTestReadPort, setTestWritePort, testRadios } from "../backend_interop/api_calls";
import { RadioTestResult } from "../backend_interop/types";
import { BackendInteropManagerContextValue, useBackendInteropManager } from "./BackendInteropManagerProvider";

const RadioTestingTab: Component = () => {
    const { availablePortNames }: BackendInteropManagerContextValue = useBackendInteropManager();

    const [isSimulating, setSimulating] = createSignal<boolean>(false);
    const [testInterval, setTestInterval] = createSignal<number>(500);
    const [secondsElapsed, setSecondsElapsed] = createSignal<number>(0);
    const [packetsAttemptedCount, setPacketsAttemptedCount] = createSignal<number>(0);
    const [packetsSentCount, setPacketsSentCount] = createSignal<number>(0);
    const [packetsRecievedCount, setPacketsRecievedCount] = createSignal<number>(0);
    const [dataLossPercent, setDataLossPercent] = createSignal<number>(0);

    const [selectedTestWritePort, setSelectedTestWritePort] = createSignal<string | null>(getTestWritePort());
    const [selectedTestReadPort, setSelectedTestReadPort] = createSignal<string | null>(getTestReadPort());

    let testTimoutId: number | undefined;

    const testRadiosAndUpdateState = async () => {
        const test_results: RadioTestResult = await testRadios();
        batch(() => {
            setSecondsElapsed(secondsElapsed() + testInterval() / 1000);
            setPacketsAttemptedCount(packetsAttemptedCount() + test_results.packets_attempted);
            setPacketsSentCount(packetsSentCount() + test_results.packets_written);
            setPacketsRecievedCount(packetsRecievedCount() + test_results.packets_read);
            setDataLossPercent(100 * (packetsRecievedCount() == 0 ? 1 : (1 - packetsRecievedCount() / packetsAttemptedCount())));
        });

        testTimoutId = window.setTimeout(testRadiosAndUpdateState, testInterval());
    };

    onCleanup(() => {
        if (testTimoutId !== undefined) {
            window.clearTimeout(testTimoutId);
        }
    });

    createEffect(() => {
        if (isSimulating()) {
            testTimoutId = window.setTimeout(testRadiosAndUpdateState, untrack(testInterval));
            batch(() => {
                setSecondsElapsed(0);
                setPacketsAttemptedCount(0);
                setPacketsSentCount(0);
                setPacketsRecievedCount(0);
                setDataLossPercent(0);
            })
        } else {
            if (testTimoutId) {
                window.clearTimeout(testTimoutId);
            }
        }
    }, { defer: true });

    createEffect(() => {
        if (selectedTestReadPort() != null) {
            setTestReadPort(selectedTestReadPort()!)
        }
    }, { defer: true });

    createEffect(() => {
        if (selectedTestWritePort() != null) {
            setTestWritePort(selectedTestWritePort()!)
        }
    }, { defer: true });

    createEffect(() => {
        if (untrack(isSimulating)) {
            if (testTimoutId) {
                window.clearTimeout(testTimoutId);
            }
            testTimoutId = window.setTimeout(testRadiosAndUpdateState, testInterval());
        }
    }, { defer: true });
    
    return (
        <div class="flex gap-4">
            <div class="flex flex-col gap-1">
                <h1 class="my-0.5 dark:text-white">Radio Connection Test</h1>
                <span class="dark:text-white">Test whether packets can be sent and received through two RFD900s connected though serial ports</span>
                <span class="dark:text-white">Packets to simulate:</span>
                <div class="flex gap-2">
                    <label for="interval-input" class="dark:text-white">Interval:</label>
                    <input type="number" value={500} onChange={(e) => setTestInterval(+(e.target as HTMLInputElement).value)} />
                    <span class="dark:text-white">milliseconds</span>
                </div>

                <div class="flex gap-2">
                    <label for="sendingRadioPortInput" class="dark:text-white">Sending Radio Port:</label>
                    <input type="text" name="Serial Port" id="sendingRadioPortInput" list="activeSerialPorts" 
                            onInput={event => setSelectedTestWritePort((event.target as HTMLInputElement).value)} value={selectedTestWritePort() ?? ""} />
                    <datalist id="activeSerialPorts">
                        <For each={availablePortNames()}>
                            {(serialPort) => <option value={serialPort.name} /> }
                        </For>
                    </datalist>
                </div>

                <div class="flex gap-2">
                    <label for="recievingRadioPortInput" class="dark:text-white">Reciving Radio Port:</label>
                    <input type="text" name="Test Port" id="recievingRadioPortInput" list="testSerialPorts" 
                            onInput={event => setSelectedTestReadPort((event.target as HTMLInputElement).value)} value={selectedTestReadPort() ?? ""} />
                    <datalist id="testSerialPorts">
                        <For each={availablePortNames()}>
                            {(serialPort) => <option value={serialPort.name} /> }
                        </For>
                    </datalist>
                </div>

                <button onClick={() => setSimulating(!isSimulating())} class={`py-2 px-8 border-rounded border-0 ${isSimulating() ? "bg-red" : "bg-green"}`}>{isSimulating() ? "Stop Test" : "Start Test"}</button>
            </div>

            <div class="flex flex-col gap-1 dark:text-white">
                <h1 class="my-0.5">Results</h1>
                <span>Elapsed time: {secondsElapsed().toFixed(2)} second{secondsElapsed() == 1 ? "" : "s"}</span>
                <span>Packets attempted: {packetsAttemptedCount()}</span>
                <span>Packets sent: {packetsSentCount()}</span>
                <span>Packets received: {packetsRecievedCount()}</span>
                <div class="flex gap-1 items-center">
                    <span>Data loss:</span>
                    <span class={`border-rounded p-1 ${secondsElapsed() != 0 ? (dataLossPercent() == 0 ? "bg-green" : (dataLossPercent() < 20 ? "bg-orange" : "bg-red")) : "bg-gray-300 dark:bg-gray-500"}`}>{dataLossPercent().toFixed(2)}%</span>
                </div>
            </div>
        </div>
    );
};

export default RadioTestingTab;