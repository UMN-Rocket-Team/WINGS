import {Component, For, JSX} from "solid-js";
import { DisplayStruct, displays } from "../components/GraphSettingsScreen";
import SolidChart from "../components/SolidChart";

const DisplayTab : Component = (): JSX.Element => {
    return (
        <div class="flex flex-col flex-grow gap-4 border-rounded dark:text-white">
                {/*Views*/}
                <div class="grid gap-2 h-100%" style={{"grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, displays.length)}, 1fr)`}}>
                    <For each={displays}>
                        {(display: DisplayStruct) =>
                            <div class="relative" style={{height: '40vh'}}>
                                <display.displayElement {...display}/>
                            </div>
                        }
                    </For>
                </div>
        </div>
    );
};
export default DisplayTab;
