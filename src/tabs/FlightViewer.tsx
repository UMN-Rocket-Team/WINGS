import { Component } from "solid-js";
import DataTab from "./DataTab";
import PacketsTab from "./PacketsTab";
import TabView from "./TabView";
import TestingTab from "./TestingTab";
import ThemeSwitcher from "../components/ThemeSwitcher";
import SendingTab from "./SendingTab";
import GraphTab from "./GraphTab";
import RadioTestingTab from "./RadioTestingTab";

/**
 * A list of components that are the tabs to display inside the flight viewer 
 */
const tabs: Component[] = [
    DataTab,
    RadioTestingTab,
    SendingTab,
    GraphTab
];

/**
 * A list of tab names corresponding to each tab in {@link tabs} to display inside the flight viewer 
 */
const tabNames: string[] = [
    "Settings",
    "Devices",
    "Transmission",
    "Display",
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