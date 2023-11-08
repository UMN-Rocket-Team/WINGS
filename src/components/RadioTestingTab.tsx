import { batch, Component, createEffect, createSignal, For, onCleanup, onMount, Show, untrack } from "solid-js";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { BackendContextValue, useBackend } from "./BackendProvider";
import { useModal } from "./ModalProvider";
import ErrorModal from "./ErrorModal";
import { startRadioTest, stopRadioTest } from "../backend_interop/api_calls";
import { RadioTestSendingState } from "../backend_interop/types";

/**
 * A component that allows the user to send test packets over a radio.
 */
const RadioTestingTab: Component = () => {
    const {availablePortNames, parsedPacketCount} = useBackend();
    const {showModal} = useModal();

    let initialPacketCount = parsedPacketCount();

    const [isSimulating, setSimulating] = createSignal(false);
    const [sendPort, setSendPort] = createSignal('');
    const [sendInterval, setSendInterval] = createSignal(500);
    const [sendingState, setSendingState] = createSignal<RadioTestSendingState | null>(null);

    const startSimulating = async () => {
        batch(() => {
            initialPacketCount = parsedPacketCount();
            setSimulating(true);
            setSendingState(null);
        });
        try {
            await startRadioTest(sendPort(), sendInterval());
        } catch (error) {
            setSimulating(false);
            showModal(ErrorModal, {
                error: 'Failed to start simulation',
                description: '' + error
            });
        }
    };

    const stopSimulating = async () => {
        await stopRadioTest();
        setSimulating(false);
    };

    let unlistenFunction: UnlistenFn;
    onMount(async () => {
        unlistenFunction = await listen<RadioTestSendingState>("radio-test-send-update", ({payload}) => {
            setSendingState(payload);
        });
    });
    onCleanup(() => {
        unlistenFunction();
    });

    return (
        <div class="flex gap-4 flex-grow">
            <div class="flex flex-col gap-2 flex-grow">
                {/* This isn't visible, just used for autocomplete on the port selectors */}
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

                <label class="flex gap-1">
                    <span>Send a packet every:</span>
                    <input class="dark:border-gray-4 border-1 border-rounded flex-grow" list="radioTestAvailablePorts"
                        type="number"
                        min={0}
                        value={sendInterval()}
                        onBeforeInput={e => {
                            // Deny any non-number characters
                            if (e.data?.match(/[^0-9]/) ?? false) {
                                e.preventDefault();
                            }
                        }}
                        onChange={e => {
                            const el = e.target as HTMLInputElement;
                            // HTML min= is not actually enforced, so we have to enforce it ourselves
                            const val = el.value.trim() === '' ? 500 : Math.max(0, +el.value);
                            el.value = val.toString();
                            setSendInterval(val);
                        }}
                        disabled={isSimulating()} />
                    <span>ms</span>
                </label>

                <button
                    class="py-2 px-8 border-rounded border-0"
                    classList={{
                        "bg-red": isSimulating(),
                        "bg-green": !isSimulating()
                    }}
                    onClick={() => isSimulating() ? stopSimulating() : startSimulating()}
                    disabled={!sendPort()}
                >
                    {isSimulating() ? "Stop Test" : "Start Test"}
                </button>

                <Show when={sendingState() !== null}>
                    <div>Sent {sendingState()?.packetsSent} packets</div>
                    <div>Received {parsedPacketCount() - initialPacketCount} packets</div>
                </Show>
            </div>
        </div>
    );
};

export default RadioTestingTab;