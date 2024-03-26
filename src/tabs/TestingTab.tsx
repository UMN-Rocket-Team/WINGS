import { Component } from "solid-js";
import RadioTestingTab from "./RadioTestingTab";
import TabView from "../components/TabView";

/**
 * A list of components that are the tabs to display inside the testing tab
 */
const tabs = [
    RadioTestingTab,
];

/**
 * A list of tab names corresponding to each tab in {@link tabs} to display inside the testing tab
 */
const tabNames = [
    "Radio Test",
];

/**
 * A utility component that defines the tab view part of the user interface, which is composed of
 * {@link tabs} inside a {@link TabView}.
 */
const TestingTab: Component = () => {
    return (
        <TabView tabs={tabs} tabNames={tabNames} containerClasses="drop-shadow-lightgray dark:drop-shadow-gray" navbarClasses="border-gray border-b-1" />
    );
};

export default TestingTab;