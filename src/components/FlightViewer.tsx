import { Component, createEffect } from "solid-js";
import { BackendInteropManagerContextValue, useBackendInteropManager } from "./BackendInteropManagerProvider";
import DataTab from "./DataTab";
import PacketsTab from "./PacketsTab";
import TabView from "./TabView";
import TestingTab from "./TestingTab";
import ThemeSwitcher from "./ThemeSwitcher";

const tabs: Component[] = [
    DataTab,
    PacketsTab,
    TestingTab,
];

const tabNames: string[] = [
    "Data",
    "Packets",
    "Testing"
];

const FlightViewer: Component = () => {
    const { newParsedPackets }: BackendInteropManagerContextValue = useBackendInteropManager();

    createEffect(() => {
        // TODO: when the newParsedPackets change, update the graphs with the new PacketData
        console.log("FlightViewer effect parsed packets: ", newParsedPackets());
    }, { defer: true });

    return (
       <TabView tabs={tabs} tabNames={tabNames} navbarClasses="drop-shadow-lightgray dark:drop-shadow-gray">
            <ThemeSwitcher />
        </TabView>
    );
};

export default FlightViewer;