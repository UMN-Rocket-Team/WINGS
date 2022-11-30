import {Component, For, JSX} from "solid-js";
import {PacketField, PacketStructure} from "../backend_interop/types";

const PacketTab: Component<PacketStructure> = (props: PacketStructure): JSX.Element => {
    return (
        <div class="flex flex-col bg-gray">
            <p class="px-2">{props.name}</p>
            <For each={props.fields}>
                {(field: PacketField) =>
                    <div class="flex">
                        <p class="text-center px-2 py-2 gap-2 bg-white dark:bg-black">{field.name}</p>
                    </div>
                }
            </For>
        </div>
    );
};

export default PacketTab;