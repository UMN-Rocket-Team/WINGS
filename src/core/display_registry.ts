import { Component } from "solid-js";
import { ModalProps } from "./ModalProvider";
import { SettingsModalProps } from "../components/DisplaySettingsScreen";
import Boolean from "../components/Boolean";
import GraphSettingsModal, { GraphStruct } from "../modals/GraphSettingsModal";
import GraphDisplayElement from "../components/SolidChart";
import BooleanSettingsModal, { BooleanStruct } from "../modals/BooleanSettingsModal";
import ReadoutSettingsModal, { ReadoutStruct } from "../modals/ReadoutSettingsModal";
import ReadoutDisplayElement from "../components/Readout";
import TemplateSettingsModal, { TemplateStruct } from "../modals/TemplateSettingsModal";
import TemplateDisplayElement from "../components/TemplateDisplayComponent";
import OscilloscopeGraphSettingsModal, { OscilloscopeGraphStruct } from "../modals/OscilloscopeGraphSettingsModal";
import OscilloscopeGraphDisplayElement from "../components/OscilloscopeChart";
import RocketSettingsModal, { RocketStruct } from "../modals/RocketSettingsModal";
import RocketElement from "../components/Rocket";

/**
 * contains all of the "settings" data that a displayType needs, this is edited by the modal, and read by the displayComponent
 */
export abstract class DisplayStruct {

    // Use this value as the key string in the displayRegistry
    abstract readonly type: string;

    // User defined name for this component
    displayName = "Unnamed";

    // packet structure id of that packet type that is being display
    packetID = 1;

    // a list of what packets have been "opened" on the SettingsModal
    packetsDisplayed: boolean[] = [false];
}

/**
 * DisplayTypeDefinition
 * contains of all elements of a display type including the JSX components
 */
export interface DisplayTypeDefinition {

    // Use this value as the key string in the displayRegistry
    readonly type: string;

    // How the frontend will be referring to this element
    displayName: string;

    // Returns a new struct for this display element
    structClass: new () => DisplayStruct;

    // This is the modal component that should be displayed to edit the Display Struct 
    //
    // Settings ModalProps is just a DisplayStruct, along with the index of the displayStruct within the array that it is stored
    settingsModal: Component<ModalProps<SettingsModalProps>>;

    // Display Component is the JSX component that will actually be on the display tab 
    displayComponent: Component<DisplayStruct>;
}

/**
 * displayRegistry is a Map of all display types to their Type definitions
 * use this to easily get access to all of the information about a specific display type
 */
export const displayRegistry = new Map<string, DisplayTypeDefinition>();
  

// Example of registering a new class
//
// displayRegistry.set("template", {
//     type: "template",
//     displayName: "Template",
//     structClass: TemplateStruct,
//     settingsModal: TemplateSettingsModal,
//     displayComponent: TemplateDisplayElement as Component<DisplayStruct>
// });

displayRegistry.set("graph", {
    type: "graph",
    displayName: "Graph",
    structClass: GraphStruct,
    settingsModal: GraphSettingsModal,
    displayComponent: GraphDisplayElement as Component<DisplayStruct>
});

displayRegistry.set("readout", {
    type: "readout",
    displayName: "Readout",
    structClass: ReadoutStruct,
    settingsModal: ReadoutSettingsModal,
    displayComponent: ReadoutDisplayElement as Component<DisplayStruct>
});

displayRegistry.set("indicator", {
    type: "indicator",
    displayName: "Indicator",
    structClass: BooleanStruct,
    settingsModal: BooleanSettingsModal,
    displayComponent: Boolean as Component<DisplayStruct>
});

displayRegistry.set("oscilloscopeGraph", {
    type: "oscilloscopeGraph",
    displayName: "Oscilloscope Graph", 
    structClass: OscilloscopeGraphStruct,
    settingsModal: OscilloscopeGraphSettingsModal,
    displayComponent: OscilloscopeGraphDisplayElement as Component<DisplayStruct>
});

displayRegistry.set("rocket", {
    type: "rocket",
    displayName: "Rocket",
    structClass: RocketStruct,
    settingsModal: RocketSettingsModal,
    displayComponent: RocketElement as Component<DisplayStruct>
});
