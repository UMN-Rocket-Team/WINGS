import { Component, batch, createSignal, JSX, For, Show } from "solid-js";
import { useBackend } from "../backend_interop/BackendProvider";
import { setActivePort, setTestPort, startSendingLoop, stopSendingLoop} from "../backend_interop/api_calls";
import ErrorModal from "../modals/ErrorModal";
import { useModal } from "../modals/ModalProvider";
import { SendingModes } from "../backend_interop/types";
import { createStore } from "solid-js/store";
const [sendPort, setSendPort] = createSignal('');
const [selectedDevice, setSelectedDevice] = createSignal<string | null>();
//const [comDevices, setComDevices] = createStore<DisplayComDevice[]>([])
const [sendInterval, setSendInterval] = createSignal(500);

const [isSimulating, setSimulating] = createSignal(false);

export const [mode, selectMode] = createSignal(SendingModes.FromCSV);

const SendingTab: Component = () => {
    const {availableDeviceNames: availablePortNames, parsedPacketCount, sendingLoopState} = useBackend();
    const { showModal } = useModal();
    const startSimulating = async () => {
        debugger;
        batch(() => {
            setSimulating(true);
        });

        try {
            await setTestPort(sendPort());
            switch (sendingLoopState()?.packetsSent){
                case undefined:
                    await startSendingLoop(sendInterval(),0 , mode());
                default:
                    await startSendingLoop(sendInterval(),sendingLoopState()?.packetsSent as number, mode());
            }
        } catch (error) {
            setSimulating(false);
            showModal(ErrorModal, {
                error: 'Failed to start simulation',
                description: '' + error,
            });
        }
    };

    const stopSimulating = async () => {
        await stopSendingLoop();
        await setTestPort('');
        setSimulating(false);
    };

    async function applyNewSelectedPort(newSelectedDevice: string) {
        // Apply the change in selected port name to the backend
        try {
            setSelectedDevice(newSelectedDevice);
            await setActivePort(newSelectedDevice);
        } catch (error) {
            showModal(ErrorModal, {error: 'Failed to set the active serial port', description: `${error}`});
        }
    }

    return (
        <div class = "flex flex-grow gap-4">
            <div class="flex flex-grow flex-col gap-4">

                <label for="DeviceInput" class="px-2 m-0">
                    <span>Device:</span>
                    <input name="Device" id="DeviceInput" class="w-50"
                        list="dataDevices" value={selectedDevice() ?? ""}
                        onChange={event => applyNewSelectedPort((event.target as HTMLInputElement).value)} />
                </label>
                
                <datalist id="dataDevices">
                    <For each={availablePortNames()}>
                        {(Device) => <option value={Device.name}/>}
                    </For>
                </datalist>
            </div>
            <div class="flex flex-grow flex-col gap-4">
                <datalist id="radioTestAvailablePorts">
                    <For each={availablePortNames()}>
                        {(serialPort) => <option value={serialPort.name} />}
                    </For>
                </datalist>
                <label class="flex gap-1">
                        <span>Sending radio port:</span>
                        <input class="dark:border-gray-4 border-1 border-rounded flex-grow" list="radioTestAvailablePorts"
                            value={sendPort() ?? ""}
                            onChange={event => setSendPort((event.target as HTMLInputElement).value)}
                            disabled={isSimulating()} />
                </label>
                <label class="flex gap-1 items-center">
                    <span>Sending a packet every:</span>
                    <input
                        class="dark:border-gray-4 border-1 border-rounded flex-grow px-2 py-1"
                        type="number"
                        min={0}
                        value={sendInterval()}
                        onBeforeInput={(e) => {
                            // Deny any non-number characters
                            if (e.data?.match(/[^0-9]/) ?? false) {
                                e.preventDefault();
                            }
                        }}
                        onChange={(e) => {
                            const el = e.target as HTMLInputElement;
                            // HTML min= is not actually enforced, so we have to enforce it ourselves
                            const val = el.value.trim() === '' ? 500 : Math.max(0, +el.value);
                            el.value = val.toString();
                            setSendInterval(val);
                        }}
                    />
                    <span>ms</span>
                </label>
                <label> Select Mode:</label>
                <select value = {mode()} onChange={e => selectMode((e.currentTarget as HTMLSelectElement).value as SendingModes)}>
                    <For each={Object.values(SendingModes).filter(k => isNaN(Number(k)))}>
                        {(mode) => <option value={mode}>{mode}</option>}
                    </For>
                </select>
                <button
                    class="py-2 px-4 border-rounded border-0 color-black"
                    classList={{
                        "bg-red": isSimulating(),
                        "bg-green": !isSimulating(),
                    }}
                    onClick={() => (isSimulating() ? stopSimulating() : startSimulating())}
                >
                    {isSimulating() ? "Stop Sending" : "Start Sending"}
                </button>
            </div>
            <div class="flex flex-grow flex-col gap-4">
                <p><b>Sent: </b>{sendingLoopState()?.packetsSent} packets</p>
                <p><b>Received: </b>{parsedPacketCount()} packets</p>
            </div>
        </div>
    );
};

export default SendingTab;