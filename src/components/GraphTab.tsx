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

const GraphTab = (props: ModalProps<GraphProps>): JSX.Element => {
    return (
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
            <div class="flex flex-grow h-0">
                {/*Views*/}
                <div class="grid grid-cols-1 p-2 gap-2" style={{ "width": "100%" }}>
                    <For each={props.selectedFields}>
                        {(fieldInPacket: GraphStruct) =>
                            <div class="relative">
                                <SolidChart fieldInPacket={fieldInPacket} />
                            </div>
                        }
                    </For>
                    {/* <FieldsScreen number={1} /> */}
                </div>
            </div>

        </div>
    );


};
export default GraphTab;
