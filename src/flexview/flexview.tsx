import { Accessor, Component, createEffect, createSignal, For, JSX, Match, onCleanup, onMount, Setter, Show, Switch } from "solid-js";
import { produce } from "solid-js/store";

export interface FlexviewFactory {
    name: string;
    component: Component<any>;
}

/**
 * An instance of a JSX element in the flexview
 */
export interface FlexviewElement {
    id: number;
    type: "element";
    element: JSX.Element;
}

export enum LayoutDirection {
    Row = "row",
    Column = "column"
}

/**
 * A child layout in the flexview
 */
export interface FlexviewLayout {
    id: number;
    type: "layout";
    /**
     * Indexes of items inside the layout, in order.
     */
    children: number[];
    /**
     * Relative size of each child. Same order as children. Should sum up to 1.
     */
    weights: number[];
    direction: LayoutDirection;
}

export type FlexviewObject = FlexviewElement | FlexviewLayout;

/**
 * Draggable handle between adjacent items in a flexview.
 */
const DragHandle: Component<{
    direction: LayoutDirection;
    layoutId: number;
    afterObjectId: number;
    objects: Array<FlexviewObject>;
    setObjects: Setter<Array<FlexviewObject>>;
    dimensions: Accessor<DOMRect>;
}> = (props) => {
    const [dragging, setDragging] = createSignal(false);

    let startX = 0;
    let startY = 0;
    let startWeights: number[] = [];

    const onMouseMove = (e: MouseEvent) => {
        props.setObjects(produce(objects => {
            const layout = objects[props.layoutId] as FlexviewLayout;

            const mouseDelta = layout.direction === LayoutDirection.Row ? (
                (e.clientX - startX) / props.dimensions().width
            ) : (
                (e.clientY - startY) / props.dimensions().height
            );

            const newWeights = startWeights.slice();
            const index = layout.children.indexOf(props.afterObjectId);

            let clampedDelta;
            if (newWeights[index] + mouseDelta < 0) {
                clampedDelta = -newWeights[index];
            } else if (newWeights[index + 1] - mouseDelta < 0) {
                clampedDelta = newWeights[index + 1];
            } else {
                clampedDelta = mouseDelta;
            }
            newWeights[index] += clampedDelta;
            newWeights[index + 1] -= clampedDelta;
            layout.weights = newWeights;
        }));
    };

    const onMouseUp = (e: MouseEvent) => {
        setDragging(false);
    };

    createEffect(() => {
        if (dragging()) {
            document.addEventListener("mousemove", onMouseMove);
            document.addEventListener("mouseup", onMouseUp);
        } else {
            document.removeEventListener("mousemove", onMouseMove);
            document.removeEventListener("mouseup", onMouseUp);
        }
    });

    return (
        <div
            style={{
                width: props.direction === LayoutDirection.Column ? "100%" : "4px",
                height: props.direction === LayoutDirection.Column ? "4px" : "100%",
                cursor: props.direction === LayoutDirection.Column ? "ns-resize" : "ew-resize"
            }}
            class="hover:bg-blue-500"
            classList={{
                // need to specify both of these to override default hover style, needed
                // as during dragging the mouse is not consistently hovering the bar
                "bg-purple-500": dragging(),
                "hover:bg-purple-500": dragging()
            }}
            onMouseDown={(e) => {
                e.preventDefault();
                startX = e.clientX;
                startY = e.clientY;
                startWeights = (props.objects[props.layoutId] as FlexviewLayout).weights;
                setDragging(true);
            }}
        >
        </div>
    );
};

interface DraggableThing {
    removeFromOldPosition: () => void;

    /**
     * Actually creates the object if it hasn't been created yet.
     * @returns The ID of the object.
     */
    actualizeObjectID: () => number;
}

interface DropTarget {
    rect: DOMRect;
    setIsHovered: Setter<boolean>;
    onAccept: (objectId: number) => void;
}

/**
 * A spot where an element can be dropped.
 */
const DropSite: Component<{
    direction: LayoutDirection;
    dropTargets: Array<() => DropTarget>;
    onAccept: (objectId: number) => void;
}> = (props) => {
    const [isHovered, setIsHovered] = createSignal(false);

    let element: HTMLElement | null = null;
    const getDropTarget = (): DropTarget => ({
        rect: element!.getBoundingClientRect(),
        setIsHovered,
        onAccept: props.onAccept
    });

    onMount(() => {
        props.dropTargets.push(getDropTarget);
    });

    onCleanup(() => {
        const idx = props.dropTargets.indexOf(getDropTarget);
        if (idx !== -1) {
            props.dropTargets.splice(idx, 1);
        }
    });

    return (
        <div
            ref={el => {
                element = el;
            }}
            class="position-relative"
            style={{
                width: props.direction === LayoutDirection.Column ? "100%" : "0",
                height: props.direction === LayoutDirection.Column ? "0" : "100%",
            }}    
        >
            <div
                class="position-relative"
                classList={{
                    'bg-red-500': isHovered()
                }}
                style={{
                    width: props.direction === LayoutDirection.Column ? "100%" : "4px",
                    height: props.direction === LayoutDirection.Column ? "4px" : "100%",
                    cursor: props.direction === LayoutDirection.Column ? "ns-resize" : "ew-resize"
                }}    
            />
        </div>
    );
};

/**
 * Render an item in the flexview. If the item is a layout, then this will be recursive.
 */
const RecursiveItemView: Component<{
    editable: Accessor<boolean>;
    objects: Array<FlexviewObject>;
    setObjects: Setter<Array<FlexviewObject>>;
    objectIndex: number;
    dropTargets: Array<() => DropTarget>;
}> = (props) => {
    const [dimensions, setDimensions] = createSignal(new DOMRect());
    const resizeObserver = new ResizeObserver((changes) => {
        for (const change of changes) {
            setDimensions(change.contentRect);
        }
    });
    onCleanup(() => {
        resizeObserver.disconnect();
    });

    const me = props.objects[props.objectIndex];

    const acceptLayoutDropItem = (index: number, objectId: number) => {
        props.setObjects(produce(objects => {
            const layout = objects[props.objectIndex] as FlexviewLayout;

            const newWeight = 1 / (layout.children.length + 1);
            const offset = newWeight / layout.children.length;
            for (let i = 0; i < layout.weights.length; i++) {
                layout.weights[i] -= offset;
            }
            layout.weights.splice(index, 0, newWeight);
            layout.children.splice(index, 0, objectId);

            return objects;
        }));
    };

    return (
        <div
            class="w-full h-full overflow-hidden"
            ref={(element) => {
                resizeObserver.observe(element);
            }}
        >
            <Switch fallback={(
                <h1>Unknown object type: {me.type}</h1>
            )}>
                <Match when={me.type === 'layout'}>
                    <div
                        class="flex w-full h-full"
                        style={{
                            "flex-direction": (me as FlexviewLayout).direction,
                        }}
                    >
                        <Show when={props.editable()}>
                            <DropSite
                                direction={(me as FlexviewLayout).direction}
                                dropTargets={props.dropTargets}
                                onAccept={(object) => {
                                    acceptLayoutDropItem(0, object);
                                }}
                            />
                        </Show>

                        {/* TODO: when empty, entire layout should be DropSite */}
                        <Show when={(me as FlexviewLayout).children.length === 0}>
                            <p class="flex align-center justify-center">(Empty layout)</p>
                        </Show>

                        <For each={(me as FlexviewLayout).children}>{(childObject, index) => (
                            <>
                                <div
                                    style={{
                                        width: (me as FlexviewLayout).direction === LayoutDirection.Row ? `${(me as FlexviewLayout).weights[index()] * 100}%` : '100%',
                                        height: (me as FlexviewLayout).direction === LayoutDirection.Row ? '100%' : `${(me as FlexviewLayout).weights[index()] * 100}%`,
                                    }}
                                >
                                    <RecursiveItemView
                                        objectIndex={childObject}
                                        objects={props.objects}
                                        setObjects={props.setObjects}
                                        editable={props.editable}
                                        dropTargets={props.dropTargets}
                                    />
                                </div>

                                <Show when={props.editable() && index() !== (me as FlexviewLayout).children.length}>
                                    <DragHandle
                                        direction={(me as FlexviewLayout).direction}
                                        layoutId={props.objectIndex}
                                        afterObjectId={childObject}
                                        objects={props.objects}
                                        setObjects={props.setObjects}
                                        dimensions={dimensions}
                                    />
                                </Show>

                                <Show when={props.editable()}>
                                    <DropSite
                                        direction={(me as FlexviewLayout).direction}
                                        dropTargets={props.dropTargets}
                                        onAccept={(object) => {
                                            // Drop after current item
                                            acceptLayoutDropItem(index() + 1, object);
                                        }}
                                    />
                                </Show>
                            </>
                        )}</For>
                    </div>
                </Match>

                <Match when={me.type === 'element'}>
                    {(me as FlexviewElement).element}
                </Match>
            </Switch>
        </div>
    )
};

const Draggable: Component<{
    name: string;
    thing: () => DraggableThing;
    setDragging: Setter<DraggableThing>;
}> = (props) => {
    return (
        <div
            class="user-select-none"
            draggable={true}
            onDragStart={() => {
                props.setDragging(props.thing);
            }}
        >
            {props.name}
        </div>
    );
};

const Factory: Component<{
    name: string;
    createObject: (id: number) => FlexviewObject;
    objects: Array<FlexviewObject>;
    setObjects: Setter<FlexviewObject[]>;
    setDragging: Setter<DraggableThing | null>;
}> = (props) => {
    return (
        <Draggable
            name={props.name}
            thing={() => ({
                removeFromOldPosition: () => {
                    // Does not exist yet, nothing to remove
                },
                actualizeObjectID: () => {
                    // Create the object to get its ID
                    const newId = props.objects.length;
                    const newObject = props.createObject(newId);
                    props.setObjects([...props.objects, newObject]);
                    return newId;
                }
            })}
            setDragging={props.setDragging}
        />
    )
};

/**
 * Customizable, recursive, flexible viewer & editor
 */
const Flexview: Component<{
    editable: Accessor<boolean>;
    objects: Array<FlexviewObject>;
    setObjects: Setter<Array<FlexviewObject>>;
    factories: Array<FlexviewFactory>;
}> = (props) => {
    const [dragging, setDragging] = createSignal<DraggableThing | null>(null);
    const dropTargets: Array<() => DropTarget> = [];
    let actualizedDropTargets: DropTarget[] = [];

    createEffect(() => {
        if (dragging()) {
            actualizedDropTargets = dropTargets.map(i => i());
        }
    });

    const pointDistance = (x1: number, y1: number, x2: number, y2: number): number => {
        return Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2);
    };

    const rectangleDistance = (rect: DOMRect, x: number, y: number): number => {
        if (x >= rect.x && x <= rect.x + rect.width) {
            // Only consider y-axis
            return Math.abs(rect.y - y);
        }

        if (y >= rect.y && y <= rect.y + rect.height) {
            // Only consider x-axis
            return Math.abs(rect.x - x);
        }

        // Consider each corner
        return Math.min(
            pointDistance(x, y, rect.x, rect.y),
            pointDistance(x, y, rect.x + rect.width, rect.y),
            pointDistance(x, y, rect.x, rect.y + rect.height),
            pointDistance(x, y, rect.x + rect.width, rect.y + rect.height),
        );
    };

    const rectanglePerimeter = (rect: DOMRect): number => {
        return rect.width * 2 + rect.height;
    };

    const getNearestTarget = (x: number, y: number): DropTarget | null => {
        let bestTarget = null;
        let bestDistance = 50;
        let bestPerimeter = Infinity;
        for (const target of actualizedDropTargets) {
            // Distance ties are broken by lowest perimeter to make it easier to select tiny lines.
            const perimeter = rectanglePerimeter(target.rect);
            const distance = rectangleDistance(target.rect, x, y);
            if (distance < bestDistance || (distance === bestDistance && perimeter < bestPerimeter)) {
                bestTarget = target;
                bestDistance = distance;
                bestPerimeter = perimeter;
            }
        }
        return bestTarget;
    };

    let lastClientX = 0;
    let lastClientY = 0;

    return (
        <div
            class="w-full h-full"
            onDragOver={(e) => {
                e.preventDefault();
                lastClientX = e.clientX;
                lastClientY = e.clientY;
                const nearest = getNearestTarget(e.clientX, e.clientY);
                for (const target of actualizedDropTargets) {
                    target.setIsHovered(target === nearest);
                }
            }}
            onDragEnd={(e) => {
                // Safari bug: e.clientX and e.clientY from the dragend event are broken
                e.preventDefault();
                const nearest = getNearestTarget(lastClientX, lastClientY);
                const draggingThing = dragging();
                if (nearest && draggingThing) {
                    draggingThing.removeFromOldPosition();
                    const id = draggingThing.actualizeObjectID();
                    nearest.onAccept(id);
                }
                for (const target of actualizedDropTargets) {
                    target.setIsHovered(false);
                }
            }}
        >
            <Show when={props.editable()}>
                <div class="flex flex-row gap-5">
                    <Factory
                        name="Horizontal Container"
                        createObject={id => ({
                            id,
                            type: 'layout',
                            direction: LayoutDirection.Row,
                            children: [],
                            weights: []
                        })}
                        objects={props.objects}
                        setObjects={props.setObjects}
                        setDragging={setDragging}
                    />

                    <Factory
                        name="Vertical Container"
                        createObject={id => ({
                            id,
                            type: 'layout',
                            direction: LayoutDirection.Column,
                            children: [],
                            weights: []
                        })}
                        objects={props.objects}
                        setObjects={props.setObjects}
                        setDragging={setDragging}
                    />

                    <For each={props.factories}>{(factory) => (
                        <Factory
                            name={factory.name}
                            createObject={id => ({
                                id,
                                type: 'element',
                                element: factory.component({})
                            })}
                            objects={props.objects}
                            setObjects={props.setObjects}
                            setDragging={setDragging}
                        />
                    )}</For>
                </div>
            </Show>

            <RecursiveItemView
                // Object 0 is the root layout.
                objectIndex={0}
                editable={props.editable}
                objects={props.objects}
                setObjects={props.setObjects}
                dropTargets={dropTargets}
            />
        </div>
    );
};

export default Flexview;
