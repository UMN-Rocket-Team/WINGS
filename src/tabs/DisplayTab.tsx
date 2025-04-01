/* eslint-disable solid/reactivity */
import { Component, For, JSX, Show } from "solid-js";
import { displays, FlexviewDisplay, FlexviewLayout, FlexviewObject, flexviewObjects } from "../components/DisplaySettingsScreen";
import { displayRegistry } from "../core/display_registry";

const RecursiveFlexviewViewer = (props: {
    object: FlexviewObject
}) => {
    const display = () => props.object! as FlexviewDisplay;
    const layout = () => props.object! as FlexviewLayout;
    const typeDef = () => displayRegistry.get(displays[display().index]!.type)!;
    const DisplayComponent = typeDef()?.displayComponent;

    // getting the total of all weights so that we can normalize them later
    const totalWeight = () => {
        let weightSum = 0;
        for (const w of layout().weights){
            weightSum += w
        }
        return weightSum;
    }

    return (
        <>
            <Show when={props.object!.type === 'display'}>
            <div
                class="overflow-hidden w-full h-full flex flex-shrink items-center justify-center border-2 border-gray-300 p-2"
            >
                <DisplayComponent {...displays[display().index]!} />
            </div>
            </Show>
            <Show when={props.object!.type === 'layout'}>
                <div
                    class="overflow-hidden w-full h-full flex items-stretch justify-center border-2 border-gray-600 p-2 gap-2"
                    style={{
                        "flex-direction": layout().direction
                    }}
                >
                    <Show
                        when={layout().children.length > 0}
                        fallback={(
                            <p>Empty layout</p>
                        )}
                    >
                        <For each={layout().children}>{(childObjectId, childObjectIndex) =>
                            { 
                                const weight_calc = () => `${(layout().weights[childObjectIndex()]/totalWeight()) * 100}%`;
                            return(<div
                                style={layout().direction === 'column' ? {
                                    height: weight_calc()
                                } : {
                                    width: weight_calc()
                                }}
                            >
                                <RecursiveFlexviewViewer
                                    object={flexviewObjects[childObjectId]}
                                />
                            </div>)}}</For>
                    </Show>
                </div>
            </Show>
        </>
    );
};

const DisplayTab: Component = (): JSX.Element => {
    return (
        <div class="flex flex-col flex-grow flex-shrink gap-4 rounded-lg dark:text-white">
            <RecursiveFlexviewViewer
                object={flexviewObjects[0]}
            />
        </div>
    );
};

export default DisplayTab;
