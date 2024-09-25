import { Component, batch, createSignal, JSX, For, Show } from "solid-js";
import { useBackend } from "../backend_interop/BackendProvider";
import { addAltusMetrum, addFileManager, addRfd, deleteDevice, initDevicePort, startSendingLoop, stopSendingLoop } from "../backend_interop/api_calls";
import ErrorModal from "../modals/ErrorModal";
import { useModal } from "../modals/ModalProvider";
import { SendingModes } from "../backend_interop/types";
import { createStore } from "solid-js/store";
import { Store } from "tauri-plugin-store-api";
import FileModal from "../modals/FilePathModal";

type comDevice = {
    id: number,
    selection: string,
}
export const [comDeviceSelections, setComDeviceSelections] = createStore<comDevice[]>([]);
export let comDevicesIterator = 0;
const [sendPort, setSendPort] = createSignal<string>();
const [sendInterval, setSendInterval] = createSignal(500);
const [baud, setBaud] = createSignal(57600);
const [isSimulating, setSimulating] = createSignal(false);
export const [mode, selectMode] = createSignal(SendingModes.FromCSV);

export const IterateComDevicesIterator = () => {
    return comDevicesIterator++;
}

const SendingTab: Component = () => {
    const { availableDeviceNames: availablePortNames, parsedPacketCount, sendingLoopState, comDeviceList, gotData } = useBackend();
    const { showModal } = useModal();

    const startSimulating = async () => {
        debugger;
        batch(() => {
            setSimulating(true);
        });

        try {
            switch (sendingLoopState()?.packetsSent) {
                case undefined:
                    await startSendingLoop(sendInterval(), 0, mode(), parseInt(sendPort() ?? "0"));
                default:
                    await startSendingLoop(sendInterval(), sendingLoopState()?.packetsSent as number, mode(), parseInt(sendPort() ?? "0"));
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
        await parseInt(sendPort() ?? "0");
        setSimulating(false);
    };

    const addFileDirectory = async (filePaths: string | string[] | null) => {
        if (Array.isArray(filePaths)) {
            for (const path of filePaths) {
                setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: path }]);
                addFileManager(path);
            }
        } else if (filePaths != null) {
            setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: filePaths }]);
            addFileManager(filePaths);
        }
    };

    async function applyNewSelectedPort(newSelectedDevice: string, baud: number, id: number) {
        try {
            setComDeviceSelections(device => device.id === id, "selection", () => newSelectedDevice)
            await initDevicePort(newSelectedDevice, baud, id);
        } catch (error) {
            showModal(ErrorModal, { error: 'Failed to set the active serial port', description: `${error}` });
        }
    }

    return (
        <div class="flex flex-grow gap-4">
            <div class="flex flex-grow flex-col gap-4">
                <button
                    onClick={async () => {
                        const store = new Store("persistent.dat");
                        const recentPaths = (await store.get("recentSaves") || []) as string[];
                        showModal(FileModal, {
                            pathStrings: recentPaths,
                            callBack: addFileDirectory
                        });
                    }}>
                    addPath&#40;s&#41;
                </button>
                <button onClick={() => { setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: "" }]); addRfd() }}>
                    addRfd
                </button>
                <button onClick={() => { setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: "" }]); addAltusMetrum() }}>
                    addAltusMetrum
                </button>
                <For each={comDeviceList()}>
                    {(device, device_index) =>
                        <label for="DeviceInput" class="px-2 m-0">
                            <span>{device.device_type} {device.id} Device: </span>
                            <input name="Device" id="DeviceInput" class="w-1/2" autocomplete="off"
                                list="dataDevices" value={comDeviceSelections[device_index()].selection}
                                onChange={event => applyNewSelectedPort((event.target as HTMLInputElement).value, baud(), device.id)} />
                            <button onClick={() => {
                                setComDeviceSelections(comDeviceSelections.filter((_, index) => device_index() != index));
                                deleteDevice(device.id)
                            }}>
                                X
                            </button>
                        </label>
                    }
                </For>

                <datalist id="dataDevices">
                    <For each={availablePortNames()}>
                        {(Device) => <option value={Device.name} />}
                    </For>
                </datalist>
            </div>
            <div class="flex flex-grow flex-col gap-4">
                <datalist id="radioTestAvailablePorts">
                    <For each={comDeviceList()}>
                        {(device) => <option value={device.id} />}
                    </For>
                </datalist>
                <datalist id="commonBauds">
                    <option value="4800" />
                    <option value="9600" />
                    <option value="19200" />
                    <option value="38400" />
                    <option value="57600" />
                    <option value="115200" />
                    <option value="230400" />
                    <option value="460800" />
                    <option value="921600" />
                </datalist>
                <label class="flex gap-1 items-center">
                    <span>baud:</span>
                    <input
                        class="border border-gray-400 rounded flex-grow px-2 py-1 dark:border-gray-600"
                        list="commonBauds"
                        min={0}
                        value={baud()}
                        onBeforeInput={(e) => {
                            if (e.data?.match(/[^0-9]/) ?? false) {
                                e.preventDefault();
                            }
                        }}
                        onChange={(e) => {
                            const el = e.target as HTMLInputElement;
                            const val = el.value.trim() === '' ? 57600 : Math.max(0, +el.value);
                            el.value = val.toString();
                            setBaud(val);
                        }}
                    />
                    <span>ms</span>
                </label>
                <label class="flex gap-1">
                    <span>Sending radio Device:</span>
                    <input class="border border-gray-400 rounded flex-grow dark:border-gray-600" autocomplete="off" list="radioTestAvailablePorts"
                        value={sendPort() ?? ""}
                        onChange={event => setSendPort((event.target as HTMLInputElement).value)}
                        disabled={isSimulating()} />
                </label>
                <label class="flex gap-1 items-center">
                    <span>Sending a packet every:</span>
                    <input
                        class="border border-gray-400 rounded flex-grow px-2 py-1 dark:border-gray-600"
                        type="number"
                        min={0}
                        value={sendInterval()}
                        onBeforeInput={(e) => {
                            if (e.data?.match(/[^0-9]/) ?? false) {
                                e.preventDefault();
                            }
                        }}
                        onChange={(e) => {
                            const el = e.target as HTMLInputElement;
                            const val = el.value.trim() === '' ? 500 : Math.max(0, +el.value);
                            el.value = val.toString();
                            setSendInterval(val);
                        }}
                    />
                    <span>ms</span>
                </label>
                <label>Select Mode:</label>
                <select value={mode()} onChange={e => selectMode((e.currentTarget as HTMLSelectElement).value as SendingModes)}>
                    <For each={Object.values(SendingModes).filter(k => isNaN(Number(k)))}>
                        {(mode) => <option value={mode}>{mode}</option>}
                    </For>
                </select>
                <button
                    class="py-2 px-4 rounded border-0 text-black"
                    classList={{
                        "bg-red-500": isSimulating(),
                        "bg-green-500": !isSimulating(),
                    }}
                    onClick={() => (isSimulating() ? stopSimulating() : startSimulating())}
                >
                    {isSimulating() ? "Stop Sending" : "Start Sending"}
                </button>
            </div>
            <div class="flex flex-grow flex-col gap-4">
                <p><b>Sent: </b>{sendingLoopState()?.packetsSent} packets</p>
                <p><b>Received: </b>{parsedPacketCount()} packets</p>
                <button
                    class="py-2 px-4 rounded border-0 text-black"
                    classList={{
                        "bg-red-500": !gotData(),
                        "bg-green-500": gotData(),
                    }}
                >
                    data_indicator
                </button>
            </div>
        </div>
    );
};

export default SendingTab;
