import { Accessor, createContext, createSignal, onCleanup, onMount, ParentComponent, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { pushParsedPackets } from "../backend_interop/buffers";
import { Packet, PacketViewModel, RefreshAndReadResult, SerialPortNames } from "../backend_interop/types";
import { emit, listen, UnlistenFn } from "@tauri-apps/api/event";

export type BackendInteropManagerContextValue = {
    availablePortNames: Accessor<SerialPortNames[]>,
    parsedPacketCount: Accessor<number>,
    packetViewModels: PacketViewModel[],
};

const BackendInteropManagerContext = createContext<BackendInteropManagerContextValue>({
    availablePortNames: (): SerialPortNames[] => [],
    parsedPacketCount: () => 0,
    packetViewModels: [],
});

export const BackendInteropManagerProvider: ParentComponent = (props) => {
    const [availablePortNames, setAvailablePortNames] = createSignal<SerialPortNames[]>([]);
    const [parsedPacketCount, setParsedPacketCount] = createSignal<number>(0);
    const [packetViewModels, setPacketViewModels] = createStore<PacketViewModel[]>([]);

    let unlistenFunctions: UnlistenFn[];

    onMount(async () => {
        unlistenFunctions = [
            await listen<string>("error", ({ payload: message }) => {
                console.error(message);
            }),
            await listen<RefreshAndReadResult>("serial-update", ({ payload: result }) => {
                console.log(result);
                if (result.newAvailablePortNames) {
                    setAvailablePortNames(result.newAvailablePortNames);
                }
                if (result.parsedPackets) {
                    pushParsedPackets(result.parsedPackets);
                    setParsedPacketCount(parsedPacketCount() + result.parsedPackets.length);
                }
            }),
            await listen<PacketViewModel[]>("packet-structures-update", event => {
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
            }),
        ];

        // Let the backend know that the frontend is ready to receive the initial "packet-structures-update" event
        await emit("initialized");
    });

    setInterval(() => {
        const parsedPackets: Packet[] = [
            { fieldData: [10, 20, 30, 40], structureId: 0, timestamp: parsedPacketCount() }
        ];

        pushParsedPackets(parsedPackets);
        setParsedPacketCount(parsedPacketCount() + parsedPackets.length);
    }, 1000);

    onCleanup((): void => {
        for (const unlistenFunction of unlistenFunctions) {
            unlistenFunction();
        }
    });

    const context = { availablePortNames: availablePortNames, parsedPacketCount: parsedPacketCount, packetViewModels: packetViewModels };

    return (
        <BackendInteropManagerContext.Provider value={context}>
            {props.children}
        </BackendInteropManagerContext.Provider>
    );
};

export const useBackendInteropManager = (): BackendInteropManagerContextValue => useContext(BackendInteropManagerContext);
