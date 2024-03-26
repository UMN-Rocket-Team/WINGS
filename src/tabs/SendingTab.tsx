import { Component, batch, createSignal, JSX } from "solid-js";
import { useBackend } from "../backend_interop/BackendProvider";
import { setTestPort, startSendingLoop, stopSendingLoop } from "../backend_interop/api_calls";
import ErrorModal from "../modals/ErrorModal";
import { useModal } from "../modals/ModalProvider";

const [sendPort, setSendPort] = createSignal('');
const [sendInterval, setSendInterval] = createSignal(500);

const [isSimulating1, setSimulating1] = createSignal(false);
const [isSimulating2, setSimulating2] = createSignal(false);
const [isSimulating3, setSimulating3] = createSignal(false);

const SendingTab: Component = () => {
    const { availableDeviceNames: availablePortNames, parsedPacketCount, sendingLoopState } = useBackend();
    const { showModal } = useModal();

    const startSimulating = async (setSimulating: (value: boolean) => void) => {
        debugger;
        batch(() => {
            setSimulating(true);
        });

        try {
            await setTestPort(sendPort());
            await startSendingLoop(sendInterval());
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
                    "bg-red": isSimulating1(),
                    "bg-green": !isSimulating1(),
                }}
                disabled={isSimulating2() || isSimulating3()}
                onClick={() => (isSimulating1() ? stopSimulating(setSimulating1) : startSimulating(setSimulating1))}
            >
                {isSimulating1() ? "Random Sending" : "Start Random Sending"}
            </button>
            <button
                class="py-2 px-4 border-rounded border-0 color-black"
                classList={{
                    "bg-red": isSimulating2(),
                    "bg-green": !isSimulating2(),
                }}
                disabled={isSimulating1() || isSimulating3()}
                onClick={() => (isSimulating2() ? stopSimulating(setSimulating2) : startSimulating(setSimulating2))}
            >
                {isSimulating2() ? "Random Sending" : "Start Random Sending"}
            </button>

            <button
                class="py-2 px-4 border-rounded border-0 color-black"
                classList={{
                    "bg-red": isSimulating3(),
                    "bg-green": !isSimulating3(),
                }}
                disabled={isSimulating1() || isSimulating2()}
                onClick={() => (isSimulating3() ? stopSimulating(setSimulating3) : startSimulating(setSimulating3))}
            >
                {isSimulating3() ? "Random Sending" : "Start Random Sending"}
            </button>
        </div>
    );
};

export default SendingTab;