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
import { GraphStruct, graphs } from "./FieldsScreen";
import SolidChart from "./SolidChart";

const GraphTab : Component = (): JSX.Element => {
    return (
        
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
            <div class="flex flex-grow h-0">
                {/*Views*/}
                <div class="grid grid-cols-1 p-2 gap-2" style={{ "width": "100%" }}>
                    <For each={graphs}>
                        {(fieldInPacket: GraphStruct) =>
                            <div class="relative">
                                <SolidChart graph = {fieldInPacket} />
                            </div>
                        }
                    </For>
                </div>
            </div>

        </div>
    );


};
export default GraphTab;
