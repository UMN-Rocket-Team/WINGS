import { createContext, onMount, ParentComponent, useContext } from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { store } from "../core/file_handling";
import { displayRegistry, DisplayStruct } from "../core/display_registry";

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
        const rawDisplays: unknown[]= await store.get("display") ?? [];

        const validatedDisplays = rawDisplays
          .filter((d: unknown): d is DisplayStruct => 
            typeof d === "object" && 
            d !== null && 
            "type" in d && 
            displayRegistry.has((d as DisplayStruct).type)
          )
          .map(d => {
            const typeDef = displayRegistry.get(d.type)!;
            const instance = new typeDef.structClass();
            Object.assign(instance, d);
            return instance;
          });

        setDisplays(validatedDisplays);
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