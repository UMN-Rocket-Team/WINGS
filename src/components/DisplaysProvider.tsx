import { createContext, onMount, ParentComponent, useContext } from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { DisplayStruct } from "./DisplaySettingsScreen";
import { store } from "../core/file_handling";
import { GraphStruct } from "../modals/GraphSettingsModal";
import { ReadoutStruct } from "../modals/ReadoutSettingsModal";
import { BooleanStruct } from "../modals/BooleanSettingsModal";

export type DisplaysContextValue = {
    /**
     * The list of currently loaded displays
     */
    displays: DisplayStruct[],
    /**
     * Sets displays array to new given value
     */
    setDisplays: SetStoreFunction<DisplayStruct[]>,
}

const DisplaysContext = createContext<DisplaysContextValue>({
    displays: [],
    setDisplays: () => {}
});

export const DisplaysProvider: ParentComponent = (props) => {
    const [displays, setDisplays] = createStore<DisplayStruct[]>([]);

    onMount(async () => {
        /**
         * a store of all displays currently on the frontend
         */
        let importedDisplays: DisplayStruct[] = await store.get("display") ?? [];

        //safety check to remove any non-expected display types
        for (let displayString in importedDisplays){
            let display = importedDisplays[displayString];
    
            if (display.type === `Graph`){
                let graph = display as GraphStruct;
                if(graph.settingsModal !== 0 || graph.displayElement !== 0 || graph.x === undefined || graph.y === undefined || graph.colors === undefined){
                    importedDisplays.splice(importedDisplays.indexOf(display),1);
                }

            } else if (display.type === `Readout`){
                let read = display as ReadoutStruct;
                if(read.settingsModal !== 1 || read.displayElement !== 1 || read.fields === undefined){
                    importedDisplays.splice(importedDisplays.indexOf(display),1);
                }

            } else if (display.type === `Indicator`) {
                let read = display as BooleanStruct;
                if(read.settingsModal !== 2 || read.displayElement !== 2 || read.fields === undefined){
                    importedDisplays.splice(importedDisplays.indexOf(display),1);
                }
            }
            else{
                console.log(importedDisplays.indexOf(display));
                importedDisplays.splice(importedDisplays.indexOf(display),1);
            }
        }
        setDisplays(importedDisplays);
    });


    const context = {
        displays: displays,
        setDisplays: setDisplays
    }

    return(
        <DisplaysContext.Provider value={context}>
            {props.children}
        </DisplaysContext.Provider>
    );
};

/**
 * Use the displays state provided by the {@link DisplaysContext}.
 * 
 * @returns the current {@link DisplaysContextValue}
 */
export const useDisplays = (): DisplaysContextValue => useContext(DisplaysContext);