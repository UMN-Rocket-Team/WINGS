import { Accessor, Component, createEffect, createSignal, For, JSX, Match, onCleanup, onMount, Setter, Show, Switch } from "solid-js";
import { produce } from "solid-js/store";

export interface FlexviewFactory<T> {
    component: Component<T>;
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

interface FlexviewEditorProps {
    editable: Accessor<boolean>;
    factories: FlexviewFactory<any>[];
    objects: Array<FlexviewObject>;
    setObjects: Setter<Array<FlexviewObject>>;
}

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
            class="hover:bg-blue"
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

const RecursiveItemView: Component<{
    editable: Accessor<boolean>;
    objects: Array<FlexviewObject>;
    setObjects: Setter<Array<FlexviewObject>>;
    objectIndex: number;
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

    return (
        <div
            class="w-100% h-100% overflow-hidden"
            ref={(element) => {
                resizeObserver.observe(element);
            }}
        >
            <Switch fallback={(
                <h1>Unknown object type: {me.type}</h1>
            )}>
                <Match when={me.type === 'layout'}>
                    <div
                        class="flex w-100% h-100%"
                        style={{
                            "flex-direction": (me as FlexviewLayout).direction,
                        }}
                    >
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

/**
 * Customizable, recursive, flexible viewer & editor
 */
const Flexview: Component<{
    editable: Accessor<boolean>;
    objects: Array<FlexviewObject>;
    setObjects: Setter<Array<FlexviewObject>>;
}> = (props) => {
    return (
        <div class="w-100% h-100%">
            <RecursiveItemView
                // Object 0 is the root layout.
                objectIndex={0}
                editable={props.editable}
                objects={props.objects}
                setObjects={props.setObjects}
            />
        </div>
    );
};

export default Flexview;
