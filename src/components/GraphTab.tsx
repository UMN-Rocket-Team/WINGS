import {Component, For, JSX} from "solid-js";
import { GraphStruct, graphs } from "./FieldsScreen";
import SolidChart from "./SolidChart";

const GraphTab : Component = (): JSX.Element => {
    return (
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
                {/*Views*/}
                <div class="grid gap-2 h-100%" style={{"grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, graphs.length)}, 1fr)`}}>
                    <For each={graphs}>
                        {(graph: GraphStruct) =>
                            <div class="relative">
                                <SolidChart {...graph}/>
                            </div>
                        }
                    </For>
                </div>
        </div>
    );
};
export default GraphTab;
