import { Component, batch, createSignal, JSX, For, Show, createMemo } from "solid-js";
import { useBackend } from "../backend_interop/BackendProvider";
import { addAim, addAltusMetrum, addFeatherWeight, addMidwest, addFileManager, addRfd, deleteDevice, initDevicePort, startSendingLoop, stopSendingLoop } from "../backend_interop/api_calls";
import ErrorModal from "../modals/ErrorModal";
import { useModal } from "../core/ModalProvider";
import { ComDevice, ProductName, SendingModes } from "../backend_interop/types";
import { createStore } from "solid-js/store";
import { Store } from "tauri-plugin-store-api";
import FileModal from "../modals/FilePathModal";

export const [comDeviceSelections, setComDeviceSelections] = createStore<ComDevice[]>([]);
let comDevicesIterator = 0;
const [baud, setBaud] = createSignal(115200);
const [sortOrder, setSortOrder] = createSignal<'asc' | 'desc'>('asc');

// Simulation states for testing purposes
/*
const [sendPort, setSendPort] = createSignal<string>();
const [sendInterval, setSendInterval] = createSignal(500);
const [isSimulating, setSimulating] = createSignal(false);
const [mode, selectMode] = createSignal(SendingModes.FromCSV);
*/

export const IterateComDevicesIterator = () => {
    return comDevicesIterator++;
}

export const EnsureComDevicesIteratorAtLeast = (minVal: number) => {
    comDevicesIterator = Math.max(comDevicesIterator, minVal);
}

const SendingTab: Component = () => {
    const { availableDeviceNames: availablePortNames, parsedPacketCount, sendingLoopState, comDeviceList } = useBackend();
    const { showModal } = useModal();

    // Groups devices by type and sorts them based on the current sortOrder state
    const groupedDevices = createMemo(() => {

        // eslint-disable-next-line solid/reactivity
        const sorted = [...comDeviceList()].sort((a, b) =>
            sortOrder() === 'asc' ? a.id - b.id : b.id - a.id
        );

        return {
            SerialPort: sorted.filter(d => d.device_type === 'SerialPort'),
            AimXtra: sorted.filter(d => d.device_type === 'AimXtra'),
            AltusMetrum: sorted.filter(d => d.device_type === 'TeleDongle'),
            FeatherWeight: sorted.filter(d => d.device_type === 'FeatherWeight'),
        };
    });

    // ----------------------------------- SIMULATION FUNCTIONS -----------------------------------
    // For testing purposes, we can simulate sending data from a serial port
    /*
    // Initiates the simulation based on the selected mode and interval
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

    // Halts the simulation and resets relevant states
    const stopSimulating = async () => {
        await stopSendingLoop();
        await parseInt(sendPort() ?? "0");
        setSimulating(false);
    };
    */

    const addFileDirectory = async (filePaths: string | string[] | null) => {
        if (Array.isArray(filePaths)) {
            for (const path of filePaths) {
                setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: path }]);
                await addFileManager(path);
            }
        } else if (filePaths != null) {
            setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: filePaths }]);
            await addFileManager(filePaths);
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

    // ----------------------------------- DYNAMIC RENDERING & DATA ----------------------------------- 
    // addPath(), addSerialPort(), addAltusMetrum(), addAim(), addFeatherWeight()

    // Common styling for all "ADD" buttons
    const buttonClasses = "w-full text-black bg-gray-200 hover:bg-gray-400 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white";

    // Handles the "Add Path(s)" button click, showing the FileModal and passing the recent paths from the store
    async function clickAddPath() {
        const store = new Store("persistent.dat");
        const recentPaths = (await store.get("recentSaves") || []) as string[];
        showModal(FileModal, {
            pathStrings: recentPaths,
            callBack: addFileDirectory
        });
    }

    // Handles the "Add SerialPort" button click, adding a new device selection and calling the addRfd API function
    function clickAddSerialPort() {
        setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: "" }]); 
        addRfd();
    }

    // Handles the "Add AltusMetrum Product" button click, adding a new device selection and calling the addAltusMetrum API function
    function clickAddAltusMetrum() {
        setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: "" }]); 
        addAltusMetrum();  
    }

    // Handles the "Add AimXtra" button click, adding a new device selection and calling the addAim API function
    function clickAddAimXtra() {
        setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: "" }]); 
        addAim();
    }

    // Handles the "Add FeatherWeight" button click, adding a new device selection and calling the addFeatherWeight API function
    function clickAddFeatherWeight() {
        setComDeviceSelections([...comDeviceSelections, { id: comDevicesIterator++, selection: "" }]); 
        addFeatherWeight();
    }

    // Array of button data for dynamic rendering
    // label: Display text on button
    // onClick: Corresponding click handler function
    const buttonsData = [
        {label: "Add Path(s)", onClick: clickAddPath},
        {label: "Add SerialPort", onClick: clickAddSerialPort}, 
        {label: "Add AltusMetrum Product", onClick: clickAddAltusMetrum},
        {label: "Add AimXtra", onClick: clickAddAimXtra},
        {label: "Add FeatherWeight", onClick: clickAddFeatherWeight},
    ];

    // Array of column data for dynamic rendering of device lists
    // label: Column header text
    // devices: Corresponding list of devices filtered by type and sorted based on current sortOrder
    const columnsData = createMemo(() => [
        {label: "SerialPort", devices: groupedDevices().SerialPort},
        {label: "AltusMetrum", devices: groupedDevices().AltusMetrum},
        {label: "AimXtra", devices: groupedDevices().AimXtra},
        {label: "FeatherWeight", devices: groupedDevices().FeatherWeight},
    ]);

    return (
        <div class="flex flex-col md:flex-row w-full min-h-0 gap-4 p-4">
            <div class="flex flex-col h-screen md:w-1/2 min-w-0 gap-4">

                {/* Dynamically render buttons based on buttonsData array, applying common styling and respective click handlers */}
                <For each={buttonsData}>
                    {(button) => (
                        <button class={buttonClasses} onClick={button.onClick}>
                            {button.label}
                        </button>
                    )}
                </For>

                <div class="flex overflow-x-auto gap-3 pb-2 min-h-200px">
                    <For each={columnsData()}>
                        {(column) => (
                            <div class="flex flex-col w-64 flex-shrink-0">
                                <div class="flex items-center gap-1 p-2 bg-gray-100 dark:bg-gray-800 rounded border border-gray-300 dark:border-gray-600">
                                    <h4 class="text-black dark:text-white text-sm font-medium">{column.label}</h4>
                                     
                                    <button 
                                        class="px-1 py-0.5 bg-gray-200 hover:bg-gray-300 dark:bg-gray-800 dark:hover:bg-gray-700 text-black dark:text-white text-xs rounded transition-colors duration-200"
                                        onClick={() => setSortOrder(sortOrder() === 'asc' ? 'desc' : 'asc')}
                                    >
                                        {sortOrder() === 'asc' ? '↑' : '↓'}
                                    </button>
                                </div>

                                <div class="flex-1 overflow-auto space-y-1 max-h-full">
                                    <For each={column.devices}>
                                        {(device) => {
                                            const globalIndex = comDeviceList().findIndex(d => d.id === device.id);

                                            return (
                                                <div class="flex items-center gap-1 p-2 bg-white dark:bg-gray-800 rounded border border-gray-300 dark:border-gray-600">
                                                    <span class="text-black dark:text-white text-xs">{device.id}</span>

                                                    <input 
                                                        class="flex-1 px-2 py-1 bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded text-black dark:text-white text-xs placeholder-gray-500 dark:placeholder-gray-400 focus:outline-none focus:ring-1 focus:ring-blue-500 transition-colors duration-200" 
                                                        autocomplete="off"
                                                        list="dataDevices" 
                                                        value={comDeviceSelections[globalIndex]?.selection ?? ""}
                                                        placeholder="path..."
                                                        onChange={event => {
                                                            applyNewSelectedPort((event.target as HTMLInputElement).value!, baud(), device.id)
                                                        }} 
                                                    />

                                                    <button 
                                                        class="px-1 py-1 bg-gray-200 hover:bg-gray-300 dark:bg-gray-800 dark:hover:bg-gray-700 text-black dark:text-white text-xs rounded transition-colors duration-200"
                                                        onClick={() => {
                                                            deleteDevice(device.id);
                                                            setComDeviceSelections(comDeviceSelections.filter((_, index) => globalIndex != index));
                                                        }}
                                                    >
                                                        ✕
                                                    </button>
                                                </div>
                                            );
                                        }}
                                    </For>
                                </div>
                            </div>
                        )}
                    </For>
                </div>              

                <datalist id="dataDevices">
                    <For each={availablePortNames()}>
                        {(Device) => <option value={Device.name} />}
                    </For>
                </datalist>
            </div>

            <div class="flex-1"/>

            {/* 
            <div class="flex- flex-grow flex-col gap-4">
                <datalist id="radioTestAvailablePorts">
                    <For each={comDeviceList()}>
                        {(device) => <option value={device.id} />}
                    </For>
                </datalist>
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
            */}

            <div class="flex flex-col md:w-1/2 gap-4 min-w-0">
                <p><b>Sent: </b>{sendingLoopState()?.packetsSent} packets</p>
                <p><b>Received: </b>{parsedPacketCount()} packets</p>

                <button
                    class="py-2 px-4 rounded-lg border-0 text-white font-medium text-lg shadow-lg transition-all duration-200"
                    classList={{
                        "bg-red-500": parsedPacketCount() === 0,
                        "bg-green-600": parsedPacketCount() > 0,
                    }}
                >
                    data_indicator
                </button>

                <br/>

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
                            const val = el.value.trim() === '' ? 115200 : Math.max(0, +el.value);
                            el.value = val.toString();
                            setBaud(val);
                        }}
                    />

                    <span>b/s</span>
                </label>
            </div>

            <div class="flex-1"/>
        </div>
    );
};

export default SendingTab;
