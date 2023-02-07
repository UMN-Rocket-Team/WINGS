import { Accessor, createContext, createSignal, onCleanup, onMount, ParentComponent, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { refreshAvailablePortsAndReadActivePort } from "../backend_interop/api_calls";
import { pushUnparsedPackets as pushParsedPackets } from "../backend_interop/buffers";
import { PacketData, PacketViewModel, SerialPortNames } from "../backend_interop/types";
import { emit, listen } from "@tauri-apps/api/event";

/**
 * The number of milliseconds to wait between refreshing the available serial ports and reading from the active port.
 */
const REFRESH_AND_READ_INTERVAL_MILLISECONDS = 1000;

export type BackendInteropManagerContextValue = {
    availablePortNames: Accessor<SerialPortNames[]>,
    newParsedPackets: Accessor<Record<number, PacketData[]> | undefined>,
    packetViewModels: PacketViewModel[],
};

const BackendInteropManagerContext = createContext<BackendInteropManagerContextValue>({
    availablePortNames: (): SerialPortNames[] => [],
    newParsedPackets: (): Record<number, PacketData[]> | undefined => undefined,
    packetViewModels: [],
});

export const BackendInteropManagerProvider: ParentComponent = (props) => {
    const [availablePortNames, setAvailablePortNames] = createSignal<SerialPortNames[]>([]);
    const [newParsedPackets, setNewParsedPackets] = createSignal<Record<number, PacketData[]>>();
    const [packetViewModels, setPacketViewModels] = createStore<PacketViewModel[]>([]);

    let refreshIntervalId: number;
    let unlistenFunction: () => void;

    onMount(async () => {
        refreshIntervalId = window.setInterval(async (): Promise<void> => {
            const result = await refreshAvailablePortsAndReadActivePort();

            if (result.newAvailablePortNames) {
                setAvailablePortNames(result.newAvailablePortNames);
            }
            if (result.parsedPackets) {
                setNewParsedPackets(pushParsedPackets(result.parsedPackets));
            }
        }, REFRESH_AND_READ_INTERVAL_MILLISECONDS);

        unlistenFunction = await listen<PacketViewModel[]>("packet-structures-update", event => {
            console.log(event);

            for (const packetViewModel of event.payload) {
                if (packetViewModels.some(oldPacketViewModel => oldPacketViewModel.id === packetViewModel.id)) {
                    // Update the existing view model
                    setPacketViewModels(
                        oldPacketViewModel => oldPacketViewModel.id === packetViewModel.id,
                        packetViewModel
                    );
                } else {
                    // Add the new view model
                    setPacketViewModels(
                        packetViewModels.length,
                        packetViewModel
                    );
                }
            }
        });

        // Let the backend know that the frontend is ready to receive the initial "packet-structures-update" event
        await emit("initialized");
    });

    onCleanup((): void => {
        clearInterval(refreshIntervalId);
        unlistenFunction();
    });

    const context = { availablePortNames: availablePortNames, newParsedPackets: newParsedPackets, packetViewModels: packetViewModels };

    return (
        <BackendInteropManagerContext.Provider value={context}>
            {props.children}
        </BackendInteropManagerContext.Provider>
    );
};

export const useBackendInteropManager = (): BackendInteropManagerContextValue => useContext(BackendInteropManagerContext);
