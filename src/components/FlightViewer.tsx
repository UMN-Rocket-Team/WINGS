import { Component } from "solid-js";
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
    return (
       <TabView tabs={tabs} tabNames={tabNames} navbarClasses="drop-shadow-lightgray dark:drop-shadow-gray">
            <ThemeSwitcher />
        </TabView>
    );
};

export default FlightViewer;