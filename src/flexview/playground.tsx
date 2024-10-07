import { Component, createSignal } from "solid-js";
import Flexview, { FlexviewFactory, FlexviewObject, LayoutDirection } from "./flexview";
import { createStore } from "solid-js/store";

const Hello: Component<{name: string}> = (props) => <div style={{
    display: "flex",
    width: "100%",
    height: "100%",
    "align-items": "center",
    "justify-content": "center"
}}>Hello {props.name}</div>;

const FlexviewPlayground: Component<{}> = () => {
    const [editable, setEditable] = createSignal(true);
    const factories: FlexviewFactory<any>[] = [
        {
            component: Hello
        },
    ];
    const [objects, setObjects] = createStore<Array<FlexviewObject>>([
        {
            id: 0,
            type: 'layout',
            direction: LayoutDirection.Column,
            children: [1, 2, 5, 4, 3],
            weights: [0.2, 0.2, 0.2, 0.2, 0.2]
        },
        {
            id: 1,
            type: 'element',
            element: <h1 class="m-0 p-0">Element 1</h1>
        },
        {
            id: 2,
            type: 'element',
            element: <h1 class="m-0 p-0">Element 2</h1>
        },
        {
            id: 3,
            type: 'element',
            element: <h1 class="m-0 p-0">Element 3</h1>
        },
        {
            id: 4,
            type: 'element',
            element: <h1 class="m-0 p-0">Element 4</h1>
        },

        {
            id: 5,
            type: 'layout',
            direction: LayoutDirection.Row,
            children: [6, 7, 8, 9],
            weights: [0.25, 0.25, 0.25, 0.25]
        },
        {
            id: 6,
            type: 'element',
            element: <h1 class="m-0 p-0">Element 6</h1>
        },
        {
            id: 7,
            type: 'element',
            element: <h1 class="m-0 p-0">Element 7</h1>
        },
        {
            id: 8,
            type: 'element',
            element: <h1 class="m-0 p-0">Element 8</h1>
        },
        {
            id: 9,
            type: 'element',
            element: <h1 class="m-0 p-0">Element 9</h1>
        },
    ]);

    return <>
        <button onClick={() => setEditable(!editable())}>Toggle edit</button>
        <Flexview
            editable={editable}
            // factories={factories}
            objects={objects}
            setObjects={setObjects}
        />
    </>;
};

export default FlexviewPlayground;
