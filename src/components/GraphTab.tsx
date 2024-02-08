import {Component, createSignal, For, JSX, Show} from "solid-js";
import logo from "../assets/logo.png";
import {useBackend} from "./BackendProvider";
import {setActivePort} from "../backend_interop/api_calls";
import {useNavigate} from "@solidjs/router";
import {Packet} from "../backend_interop/types";
import {parsedPackets} from "../backend_interop/buffers";
import {writeFile} from "@tauri-apps/api/fs";
import {save} from "@tauri-apps/api/dialog";
import ErrorModal, {ErrorModalProps} from "./ErrorModal";
import {ModalProps, useModal} from "./ModalProvider";
import { GraphStruct, getGraphs } from "./FieldsScreen";
import SolidChart from "./SolidChart";


export type GraphProps = {
    /**
     * The list of selected packets on this screen
     */
    selectedFields: GraphStruct[];
    /**
     * The user-displayable number of this screen
     */
    number: number;
};

const GraphTab : Component = (): JSX.Element => {
    let graphs: GraphStruct[];
    return (
        <div class="absolute z-10 top-0 left-0 bottom-0 right-0 flex flex-col bg-white dark:bg-dark-700 p-4" tabIndex={-1}
            // Focus the root div of the modal when it is made visible so that it receives keyboard events.
            // The root div of the modal needs to receive keyboard events so that it can close when the Escape key is pressed
            ref={rootElement => setTimeout(() => rootElement.focus())} // Not sure why the setTimeout is necessary, but it is
            onKeyDown={event => {
                // Close the modal if the Escape key is pressed
            }}>
            {/* <b class="text-center text-4xl dark:text-white">{`Screen ${props.number}`}</b>
            <div class="grid gap-2 h-100%" style={{"grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, props.selectedFields.length)}, 1fr)`}}> */}
                {/* <For each={graphs}>
                    {(fieldInPacket: GraphStruct) =>
                        <div class="relative">
                            <SolidChart fieldInPacket={fieldInPacket} />
                        </div>
                    }
                </For>
                <Show when={graphs.length === 0}>
                    <span class="inline-flex items-center justify-center">No packets to display</span>
                </Show> */}
            {/* </div> */}
        </div>
    );


};
export default GraphTab;
