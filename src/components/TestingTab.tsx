import { Component } from "solid-js";
import RadioTestingTab from "./RadioTestingTab";
import TabView from "./TabView";

const tabs = [
    RadioTestingTab,
];

const tabNames = [
    "Radio Test",
];

const TestingTab: Component = () => {
    return (
        <TabView tabs={tabs} tabNames={tabNames} containerClasses="drop-shadow-lightgray dark:drop-shadow-gray" navbarClasses="border-gray border-b-1" />
    );
};

export default TestingTab;