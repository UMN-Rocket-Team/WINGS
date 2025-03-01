import { Component } from "solid-js";
import SettingsTab from "./SettingsTab";
import PacketEditor from "../components/PacketsEditor";
import TabView from "../components/TabView";
import ThemeSwitcher from "../theme/ThemeSwitcher";
import SendingTab from "./SendingTab";
import DisplayTab from "./DisplayTab";

/**
 * A list of components that are the tabs to display inside the flight viewer 
 */
const tabs: Component[] = [
    SettingsTab,
    SendingTab,
    DisplayTab
];

/**
 * A list of tab names corresponding to each tab in {@link tabs} to display inside the flight viewer 
 */
const tabNames: string[] = [
    "Settings",
    "Communications",
    "Display",
];

/**
 * A utility component that defines the flight viewer part of the user interface, which is composed of multiple
 * {@link tabs} inside a {@link TabView}.
 */
const TabPage: Component = () => {
    return (
        <TabView tabs={tabs} tabNames={tabNames} navbarClasses="shadow-md dark:shadow-gray-800">
            <ThemeSwitcher />
        </TabView>
    );
};

export default TabPage;
