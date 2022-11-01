import { Accessor, createContext, createSignal, onCleanup, onMount, ParentComponent, useContext } from "solid-js";
import { refreshAvailablePortsAndReadActivePort, writeTestPacketToTestPort } from "../backend_interop/api_calls";
import { pushUnparsedPackets as pushParsedPackets } from "../backend_interop/buffers";
import { PacketData, SerialPortNames } from "../backend_interop/types";

/**
 * The number of milliseconds to wait between refreshing the available serial ports and reading from the active port.
 */
const REFRESH_AND_READ_INTERVAL_MILLISECONDS = 1000;

export type BackendInteropManagerContextValue = {
    availablePortNames: Accessor<SerialPortNames[]>,
    newParsedPackets: Accessor<Record<number, PacketData[]> | undefined>,
};

const BackendInteropManagerContext = createContext<BackendInteropManagerContextValue>({
    availablePortNames: (): SerialPortNames[] => [],
    newParsedPackets: (): Record<number, PacketData[]> | undefined => undefined,
});

export const BackendInteropManagerProvider: ParentComponent = (props) => {
    const [availablePortNames, setAvailablePortNames] = createSignal<SerialPortNames[]>([]);
    const [newParsedPackets, setNewParsedPackets] = createSignal<Record<number, PacketData[]>>();

    let refreshIntervalId: number;

    onMount(async () => {
        refreshIntervalId = window.setInterval(async (): Promise<void> => {
            await writeTestPacketToTestPort();

            const result = await refreshAvailablePortsAndReadActivePort();
            
            if (result.new_available_port_names) {
                setAvailablePortNames(result.new_available_port_names);
            }
            if (result.parsed_packets) {
                setNewParsedPackets(pushParsedPackets(result.parsed_packets));
            }
        }, REFRESH_AND_READ_INTERVAL_MILLISECONDS);
    });

    onCleanup((): void => clearInterval(refreshIntervalId));

    const context = { availablePortNames: availablePortNames, newParsedPackets: newParsedPackets };

    return (
        <BackendInteropManagerContext.Provider value={context}>
            {props.children}
        </BackendInteropManagerContext.Provider>
    );
};

export const useBackendInteropManager = (): BackendInteropManagerContextValue => useContext(BackendInteropManagerContext);
