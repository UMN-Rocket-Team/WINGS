import { Component, JSX } from "solid-js";
import { ModalProps } from "./ModalProvider";
import { SettingsModalProps } from "../components/DisplaySettingsScreen";
import Boolean from "../components/Boolean";
import GraphSettingsModal, { GraphStruct } from "../modals/GraphSettingsModal";
import GraphDisplayElement from "../components/SolidChart";
import BooleanSettingsModal, { BooleanStruct } from "../modals/BooleanSettingsModal";
import ReadoutSettingsModal, { ReadoutStruct } from "../modals/ReadoutSettingsModal";
import ReadoutDisplayElement from "../components/Readout";

export abstract class DisplayStruct {
    abstract readonly type: string;
    displayName = "Unnamed";
    packetID = 1;
    settingsModal!: number;
    displayElement!: number;
    packetsDisplayed: boolean[] = [false];
    displayID?: number;
}

export interface DisplayTypeDefinition {
    readonly type: string;
    displayName: string;
    structClass: new () => DisplayStruct;
    settingsModal: Component<ModalProps<SettingsModalProps>>;
    displayComponent: Component<DisplayStruct>;
}

export const displayRegistry = new Map<string, DisplayTypeDefinition>();

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