import { Component } from "solid-js";
import GraphScreen from "./GraphScreen";

const DataTab: Component = () => {
    return (
        <div class="grid" style={{ "grid-template-columns": "1fr 1fr", "gap": "1rem", "grid-auto-rows": "1fr" }}>
            <GraphScreen />
            <GraphScreen />
            <GraphScreen />
            <GraphScreen />
        </div>
    );
};

export default DataTab;