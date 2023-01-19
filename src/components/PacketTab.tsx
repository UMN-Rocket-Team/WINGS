import {Component, For, JSX} from "solid-js";
import {PacketField, PacketStructure} from "../backend_interop/types";

const PacketTab: Component<PacketStructure> = (props: PacketStructure): JSX.Element => {
    return (
        <div class="bg-gray">
            <p class="px-2">{props.name}</p>
            <div class="flex flex-col gap-2 py-2">
                <For each={props.fields}>
                    {(field: PacketField) =>
                        <p class="mx-2 my-0 px-2 py-2 text-center bg-white dark:bg-black">{field.name}</p>
                    }
                </For>
            </div>
        </div>
    );
};

export default PacketTab;