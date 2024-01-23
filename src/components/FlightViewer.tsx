import { Component } from "solid-js";
import DataTab from "./DataTab";
import PacketsTab from "./PacketsTab";
import TabView from "./TabView";
import TestingTab from "./TestingTab";
import ThemeSwitcher from "./ThemeSwitcher";
import SendingTab from "./SendingTab";

/**
 * A list of components that are the tabs to display inside the flight viewer 
 */
const tabs: Component[] = [
    DataTab,
    PacketsTab,
    TestingTab,
    SendingTab
];

/**
 * A list of tab names corresponding to each tab in {@link tabs} to display inside the flight viewer 
 */
const tabNames: string[] = [
    "Data",
    "Packets",
    "Testing",
    "Sending"
];

/**
 * A utility component that defines the flight viewer part of the user interface, which is composed of multiple
 * {@link tabs} inside a {@link TabView}.
 */
const FlightViewer: Component = () => {
    return (
       <TabView tabs={tabs} tabNames={tabNames} navbarClasses="drop-shadow-lightgray dark:drop-shadow-gray">
            <ThemeSwitcher />
        </TabView>
    );
};

export default FlightViewer;