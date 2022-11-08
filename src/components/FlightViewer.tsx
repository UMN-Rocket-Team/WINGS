import { Component, createEffect, createSignal, For } from "solid-js";
import { Dynamic } from "solid-js/web";
import { BackendInteropManagerContextValue, useBackendInteropManager } from "./BackendInteropManagerProvider";
import DataTab from "./DataTab";
import TestingTab from "./TestingTab";
import ThemeSwitcher from "./ThemeSwitcher";

const tabNames: string[] = [
    "Data",
    "Testing"
];

const tabs: Component[] = [
    DataTab,
    TestingTab,
];

const FlightViewer: Component = () => {
    const [selectedTabIndex, setSelectedTabIndex] = createSignal<number>(0);

    const { newParsedPackets }: BackendInteropManagerContextValue = useBackendInteropManager();

    createEffect(() => {
        // TODO: when the newParsedPackets change, update the graphs with the new PacketData
        console.log("FlightViewer effect parsed packets: ", newParsedPackets());
    }, { defer: true });

    return (
        <div class="flex flex-col p-4 gap-4 dark:bg-dark-700 h-full">
            <nav class="flex p-2 justify-between drop-shadow-lightgray dark:drop-shadow-gray">
                <div class="flex gap-2">
                    <For each={tabNames}>
                        {(tabName, index) => 
                            <button data-index={index()} onClick={() => setSelectedTabIndex(index())}
                                    class={`py-2 px-8 border-rounded border-0 text-base dark:text-white ${index() === selectedTabIndex() ? "bg-blue-400 dark:bg-blue-600" : "bg-transparent"} hover:bg-blue-400 hover:dark:bg-blue-600`}>
                                        {tabName}
                            </button>
                        }
                    </For>
                </div>
                <ThemeSwitcher />
            </nav>

            <Dynamic component={tabs[selectedTabIndex()]} />
        </div>
    );
};

export default FlightViewer;