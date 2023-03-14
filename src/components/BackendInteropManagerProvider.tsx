import { Accessor, createContext, createSignal, onCleanup, onMount, ParentComponent, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { pushUnparsedPackets as pushParsedPackets } from "../backend_interop/buffers";
import { PacketData, PacketViewModel, RefreshAndReadResult, SerialPortNames } from "../backend_interop/types";
import { emit, listen, UnlistenFn } from "@tauri-apps/api/event";

export type BackendInteropManagerContextValue = {
    availablePortNames: Accessor<SerialPortNames[]>,
    newParsedPackets: Accessor<Record<number, PacketData[]> | undefined>,
    packetViewModels: PacketViewModel[],
};

const BackendInteropManagerContext = createContext<BackendInteropManagerContextValue>({
    availablePortNames: (): SerialPortNames[] => [],
    newParsedPackets: (): Record<number, PacketData[]> | undefined => undefined,
    packetViewModels: [{id:1,name:"test packet", components:[]}],
});

export const BackendInteropManagerProvider: ParentComponent = (props) => {
    const [availablePortNames, setAvailablePortNames] = createSignal<SerialPortNames[]>([]);
    const [newParsedPackets, setNewParsedPackets] = createSignal<Record<number, PacketData[]>>();
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
                    setNewParsedPackets(pushParsedPackets(result.parsedPackets));
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

    onCleanup((): void => {
        // clearInterval(refreshIntervalId);
        for (const unlistenFunction of unlistenFunctions) {
            unlistenFunction();
        }
    });

    const context = { availablePortNames: availablePortNames, newParsedPackets: newParsedPackets, packetViewModels: packetViewModels };

    return (
        <BackendInteropManagerContext.Provider value={context}>
            {props.children}
        </BackendInteropManagerContext.Provider>
    );
};

export const useBackendInteropManager = (): BackendInteropManagerContextValue => useContext(BackendInteropManagerContext);
