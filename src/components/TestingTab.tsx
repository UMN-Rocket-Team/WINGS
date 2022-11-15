import { Component } from "solid-js";
import ApiTestingTab from "./ApiTestingTab";
import RadioTestingTab from "./RadioTestingTab";
import TabView from "./TabView";

const tabs = [
    RadioTestingTab,
    ApiTestingTab,
];

const tabNames = [
    "Radio Test",
    "API Test"
];

const TestingTab: Component = () => {
    return (
        <TabView tabs={tabs} tabNames={tabNames} containerClasses="drop-shadow-lightgray dark:drop-shadow-gray" navbarClasses="border-gray border-b-1" />
    );
};

export default TestingTab;