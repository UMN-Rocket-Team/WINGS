import { Component, For, Show } from "solid-js";
import { useModal } from "../core/ModalProvider";
import { createStore } from "solid-js/store";
import { useBackend } from "../backend_interop/BackendProvider";
import settingsIcon from "../assets/settings.png";
import closeIcon from "../assets/close.svg";
import { store } from "../core/file_handling";
import { displayRegistry, DisplayStruct } from "../core/display_registry";
import { loadedDisplayData, setLoadedDisplayData } from "../tabs/Homepage";

/**
 * general set of props to give each display settingsModal
 */
export type SettingsModalProps = {
    displayStruct: DisplayStruct,
    index: number,
}

/**
 * holds all display structs for future reference
 */
export interface FlexviewDisplay {
    type: 'display'; //
    index: number; //location of the display in the displaysArray
}

export interface FlexviewLayout {
    type: 'layout';
    direction: 'column' | 'row';
    children: number[];
    weights: number[];
}

export type FlexviewObject = FlexviewDisplay | FlexviewLayout | undefined;
const displayFromStoreResult = await store.get("display") as DisplayStruct[];
// let displayFromStoreFinal = [];
export let [displays, setDisplays] = createStore<(DisplayStruct | undefined)[]>(displayFromStoreResult ?? []);
// displayFromStoreResult.then((displayFromStore) => {
//     if ((displayFromStore as any[]).length > 0) {
//         displayFromStoreFinal = displayFromStore;
//         [displays, setDisplays] = createStore<(DisplayStruct | undefined)[]>(displayFromStoreFinal);
//     }
// });

let flexViewObjectsFromStoreResult = await store.get("flexviewObjects") as FlexviewObject[];

if (!flexViewObjectsFromStoreResult || flexViewObjectsFromStoreResult.length < 1) 
    flexViewObjectsFromStoreResult = [{
        type: 'layout',
        children: [],
        weights: [],
        direction: 'row'
    }];

// let flexViewObjectsFromStoreFinal = [{
//     type: 'layout',
//     children: [],
//     weights: [],
//     direction: 'row'
// }];
export let [flexviewObjects, setFlexviewObjects] = createStore<FlexviewObject[]>(flexViewObjectsFromStoreResult);
// flexViewObjectsFromStoreResult.then((flexViewObjectsFromStore) => {
//     if ((flexViewObjectsFromStore as any[]).length > 0) {
//         flexViewObjectsFromStoreFinal = flexViewObjectsFromStore;
//         [flexviewObjects, setFlexviewObjects] = createStore<FlexviewObject[]>(flexViewObjectsFromStoreFinal);
//     }
// });

let counter = 1; //iterates to give each graph a different number in its display name ie Indicator 1, indicator 2, indicator 3

const RecursiveFlexviewEditor = (props: {
    objectIndex: number
}) => {
    const { PacketStructureViewModels } = useBackend();
    const { showModal } = useModal();

    // If user loaded a display setup from a JSON file, use that data
    if (loadedDisplayData.loadedDisplays.length > 0 || 
        loadedDisplayData.loadedFlexviewObjects.length > 0) {
        setFlexviewObjects(loadedDisplayData.loadedFlexviewObjects);
        setDisplays(loadedDisplayData.loadedDisplays);
        store.set("display", displays);
        store.set("flexviewObjects", flexviewObjects);
        setLoadedDisplayData({
            loadedFlexviewObjects: [],
            loadedDisplays: [],
        });
    }


    if (flexviewObjects[props.objectIndex]!.type === 'display') {
        const display = flexviewObjects[props.objectIndex] as FlexviewDisplay;
        return (

            <div
                class="flex items-center justify-center p-2 flex-grow"
            >
                <h1>
                    {displays[display.index]?.displayName}
                </h1>
            </div>
        );
    }

    if (flexviewObjects[props.objectIndex]!.type === 'layout') {
        const layout = flexviewObjects[props.objectIndex] as FlexviewLayout;

        // getting the total of all weights so that we can normalize them later
        const totalWeight = () => {
            let weightSum = 0;
            for (const w of layout.weights) {
                weightSum += w
            }
            return weightSum;
        }
        return (
            <div class="flex-grow flex flex-col p-2 gap-2">
                <div class="flex items-center justify-start border-2 border-gray-700 dark:border-gray-300 w-full overflow-auto p-2" >
                    {/*Element Buttons*/}
                    <For each={Array.from(displayRegistry.values())}>{(typeDef) => (
                        <button type="button" class="m-1 text-black bg-gray-300 hover:bg-slate-300 focus:outline-none focus:ring-4
                            focus:ring-gray-400 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 z-1000
                            dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white"
                            onClick={() => {
                                const newDisplay = new typeDef.structClass();
                                newDisplay.displayName = `${typeDef.displayName} ${counter}`;
                                newDisplay.packetID = PacketStructureViewModels[0].id;

                                const displayArrayIndex = displays.length;
                                const flexViewObjectsIndex = flexviewObjects.length;
                                //insert into displayArrays
                                setDisplays(displayArrayIndex, newDisplay);

                                // creating a new flexview object and pushing it to the FlexViewObjects Store
                                setFlexviewObjects(flexViewObjectsIndex, {
                                    type: 'display',
                                    index: displayArrayIndex,
                                });

                                //editing this layout in the Flexview Object Store to add the item above as its child
                                setFlexviewObjects(props.objectIndex, {
                                    type: layout.type,
                                    children: [...layout.children, flexViewObjectsIndex],
                                    weights: [...layout.weights, 1],
                                    direction: layout.direction
                                });
                                counter++;
                                store.set("display", displays);
                                store.set("flexviewObjects", flexviewObjects);
                            }}
                        >
                            New {typeDef.displayName}
                        </button>
                    )}
                    </For>

                    {/*Division Buttons*/}
                    <button type="button" class="m-1 text-black bg-gray-300 hover:bg-slate-300 focus:outline-none focus:ring-4
                        focus:ring-gray-400 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 z-1000
                        dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white"
                        onClick={() => {
                            const flexViewObjectsIndex = flexviewObjects.length;
                            // creating a new flexview layout and pushing it to the FlexViewObjects Store
                            setFlexviewObjects(flexViewObjectsIndex, {
                                type: 'layout',
                                children: [],
                                weights: [],
                                direction: 'column'
                            });

                            //editing this layout in the Flexview Object Store to add the item above as its child
                            setFlexviewObjects(props.objectIndex, {
                                type: 'layout',
                                children: [...layout.children, flexViewObjectsIndex],
                                weights: [...layout.weights, 1],
                                direction: layout.direction
                            });
                            counter++;
                            store.set("display", displays);
                            store.set("flexviewObjects", flexviewObjects);
                        }}
                    >
                        - Div
                    </button>
                    <button type="button" class="m-1 text-black bg-gray-300 hover:bg-slate-300 focus:outline-none focus:ring-4
                        focus:ring-gray-400 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 z-1000
                        dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white"
                        onClick={() => {
                            const flexViewObjectsIndex = flexviewObjects.length;
                            // creating a new flexview layout and pushing it to the FlexViewObjects Store
                            setFlexviewObjects(flexViewObjectsIndex, {
                                type: 'layout',
                                children: [],
                                weights: [],
                                direction: 'row'
                            });

                            //editing this layout in the Flexview Object Store to add the item above as its child
                            setFlexviewObjects(props.objectIndex, {
                                type: 'layout',
                                children: [...layout.children, flexViewObjectsIndex],
                                weights: [...layout.weights, 1],
                                direction: layout.direction
                            });
                            counter++;
                            store.set("display", displays);
                            store.set("flexviewObjects", flexviewObjects);
                        }}
                    >
                        | Div
                    </button>
                </div>
                <div
                    class="flex flex-grow items-stretch overflow-auto gap-2"
                    style={{
                        "flex-direction": layout.direction
                    }}
                >



                    <Show
                        when={layout.children.length > 0}
                        fallback={(
                            <p>Empty layout</p>
                        )}
                    >
                        <For each={layout.children}>{(TotalArrayObjectIndex, childArrayObjectIndex) => {
                            const weight_calc = () => `${(layout.weights[childArrayObjectIndex()] / totalWeight()) * 100}%`;
                            return (<div
                                class="border-2  border-gray-700 dark:border-gray-300 relative flex flex-col"
                                style={layout.direction === 'column' ? {
                                    height: weight_calc()
                                } : {
                                    width: weight_calc()
                                }}
                            >   <div class="flex overflow-auto">
                                    <img alt="Class" src={closeIcon} draggable={false} onClick={() => {

                                        // Editing this layout in the Flexview Object Store to remove its child.
                                        setFlexviewObjects(props.objectIndex, {
                                            type: 'layout',
                                            children: layout.children.toSpliced(childArrayObjectIndex(), 1),
                                            weights: layout.weights.toSpliced(childArrayObjectIndex(), 1),
                                            direction: layout.direction
                                        });

                                        // Removing display from the display array (if this is a display)
                                        if (flexviewObjects[TotalArrayObjectIndex]!.type === 'display') {
                                            setDisplays(((flexviewObjects[TotalArrayObjectIndex] as FlexviewDisplay).index), undefined);
                                            store.set("display", displays);
                                        }

                                        // removing this object from the FlexviewObjects
                                        setFlexviewObjects(TotalArrayObjectIndex, undefined);
                                        store.set("flexviewObjects", flexviewObjects);
                                    }}
                                        class="w-[25px] dark:invert z-[1] cursor-pointer m-5" />
                                    <Show when={layout.weights.length > 1}>
                                        <input class="w-[50px] m-5 bg-transparent border-b-2 border-black"
                                            value={layout.weights[childArrayObjectIndex()]}
                                            type="number"
                                            min="1"
                                            onChange={event => {
                                                if (Number(event.target.value) < 1)
                                                    event.target.value = "1";
                                                setFlexviewObjects(props.objectIndex, {
                                                    type: 'layout',
                                                    children: layout.children,
                                                    weights: layout.weights.toSpliced(childArrayObjectIndex(), 1, parseInt((event.target as HTMLInputElement).value)),
                                                    direction: layout.direction
                                                });
                                                store.set("flexviewObjects", flexviewObjects);
                                            }} />
                                    </Show>
                                    <Show when={flexviewObjects[TotalArrayObjectIndex]?.type == "display"}>
                                        <img alt="Settings" src={settingsIcon} draggable={false} onClick={() => {
                                            const childDisplay = flexviewObjects[TotalArrayObjectIndex] as FlexviewDisplay;
                                            return (
                                                showModal<SettingsModalProps, {}>(
                                                    displayRegistry.get(displays[childDisplay.index]!.type)!.settingsModal ?? 0, {
                                                        displayStruct: displays[childDisplay.index],
                                                        index: childDisplay.index
                                                    } as SettingsModalProps
                                                )
                                            );
                                        }}
                                            class="w-[25px] dark:invert z-[1] cursor-pointer m-5" />
                                    </Show>
                                </div>
                                <RecursiveFlexviewEditor
                                    objectIndex={TotalArrayObjectIndex}
                                />
                            </div>)
                        }}
                        </For>
                    </Show>
                </div>
            </div>
        );
    }

    return (
        <div>Unknown flexview object</div>
    );
};

/**
 * A component that:
 * - Displays a list of selected fields added to this screen
 * - Allows users to add fields to the screen
 * - Allows users to clear the screen
 * - Allows users to view the graphed data received for the selected fields
 *
 * @param props an object that contains the number of this screen
 */
const FieldsScreen: Component = () => {
    return (
        <div class=" flex flex-grow relative p-2 mb-5 w-full h-full">
            {/*Field Select Button*/}
            {/* <div>
                {Array.from(displayRegistry.values()).map((typeDef) => (
                    <button type="button" class="m-1 text-black bg-gray-100 hover:bg-gray-200 focus:outline-none focus:ring-4
                    focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 z-1000
                    dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700 dark:text-white" 
                    onClick={() => {
                        const newDisplay = new typeDef.structClass();
                        newDisplay.displayName = `${typeDef.displayName} ${counter}`;
                        newDisplay.packetID= PacketStructureViewModels[0].id;
                        setFlexviewObjects([...flexviewObjects, newDisplay]);
                        counter++;
                        store.set("display", flexviewObjects);
                    }}
                    >
                    New {typeDef.displayName}
                    </button>
                ))}
            </div> */}

            {/*Fields*/}
            {/* <div
                class="absolute grid flex-wrap top-10 bottom-8 left-0 mt-5 right-0 m-auto p-4 items-center justify-center gap-4 overflow-y-scroll"
                style={{ "width": "90%", "grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, flexviewDisplays.length)}, 1fr)`}}>
                <For each={flexviewDisplays}>
                    {(display: DisplayStruct, index) => {
                        return (
                            <div class="bg-stone-400 dark:bg-neutral-900 flex justify-center items-center h-[100px] p-1.5 overflow-hidden rounded-2xl">
                                <button 
                                    class="bg-white w-full h-full rounded-[1.375rem] border-0 justify-center dark:bg-neutral-700"
                                    onClick={() => {showModal<SettingsModalProps, {}>(displayRegistry.get(display.type)!.settingsModal ?? 0, {
                                        displayStruct: display,
                                        index:index(),
                                    } as SettingsModalProps)}
                                }>
                                    <h3 class="text-black dark:text-white">{display.displayName}</h3>
                                </button>
                            </div>
                        );
                    }}
                </For>
            </div> */}

            <RecursiveFlexviewEditor
                objectIndex={0}
            />
        </div>
    )
}

export default FieldsScreen;
