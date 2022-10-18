import { Component, createContext, createSignal, ParentComponent, Setter, useContext } from "solid-js";
import { Dynamic } from "solid-js/web";
import { MainComponentType } from "../core/main_component";

export type MainComponentContextValue<T> = {
    setMainComponentId: Setter<T>
};

const MainComponentContext = createContext<MainComponentContextValue<MainComponentType>>({
    setMainComponentId: () => { throw new Error("Cannot set main component ID in default implementation!"); }
});

export type MainComponentProviderProps<T extends string | number | symbol> = {
    components: Record<T, Component>,
    initialComponentId: T,
};

export const MainComponentProvider: ParentComponent<MainComponentProviderProps<MainComponentType>> = (props) => {
    const [mainComponentId, setMainComponentId] = createSignal<MainComponentType>(props.initialComponentId);

    return (
        <MainComponentContext.Provider value={{ setMainComponentId }}>
            <Dynamic component={props.components[mainComponentId()]} />
        </MainComponentContext.Provider>
    )
};

export const useMainComponent = (): MainComponentContextValue<MainComponentType> => useContext(MainComponentContext);
