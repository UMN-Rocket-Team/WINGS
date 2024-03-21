import { Component, batch, createSignal, JSX } from "solid-js";
import { useBackend } from "../backend_interop/BackendProvider";
import { setTestPort, startSendingLoop, stopSendingLoop} from "../backend_interop/api_calls";
import ErrorModal from "../modals/ErrorModal";
import { useModal } from "../modals/ModalProvider";
import { SendingModes } from "../backend_interop/types";
const [sendPort, setSendPort] = createSignal('');
const [sendInterval, setSendInterval] = createSignal(500);

const [isSimulating, setSimulating] = createSignal(false);

const [settingMode, selectMode] = createSignal(SendingModes.FromCSV);

const SendingTab: Component = () => {
    const { availablePortNames, parsedPacketCount, sendingLoopState} = useBackend();
    const { showModal } = useModal();
    const startSimulating = async (setSimulating: (value: boolean) => void) => {
        debugger;
        batch(() => {
            setSimulating(true);
        });

        try {
            await setTestPort(sendPort());
            await startSendingLoop(sendInterval(),settingMode());
        } catch (error) {
            setSimulating(false);
            showModal(ErrorModal, {
                error: 'Failed to start simulation',
                description: '' + error,
            });
        }
    };

    const stopSimulating = async (setSimulating: (value: boolean) => void) => {
        await stopSendingLoop();
        await setTestPort('');
        setSimulating(false);
    };

    return (
        <div class="flex flex-col gap-4">
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
            <button
                class="py-2 px-4 border-rounded border-0 color-black"
                classList={{
                    "bg-red": isSimulating(),
                    "bg-green": !isSimulating(),
                }}
                onClick={() => (isSimulating() ? stopSimulating(setSimulating) : startSimulating(setSimulating))}
            >
                {isSimulating() ? "Random Sending" : "Start Random Sending"}
            </button>

            <label> Select Mode:</label>
            <select value = {settingMode()} onChange={e => selectMode((e.target as HTMLSelectElement).value as SendingModes)}>
                <option value={SendingModes.FromCSV}>From CSV</option>
                <option value={SendingModes.AllOnes}>All Ones</option>
                <option value={SendingModes.AllZeroes}>All Zeroes</option>
                <option value={SendingModes.Alternating}>Alternating</option>
                <option value={SendingModes.TimeStampAndIncreasing}>Time Stamp and Increasing</option>
            </select>
        </div>
    );
};

export default SendingTab;