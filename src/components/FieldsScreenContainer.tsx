import { Component, For } from "solid-js";
import FieldsScreen from "./FieldsScreen";

/**
 * A component that contains four {@link FieldsScreen} components arranged in a 2x2 grid.
 * 
 * @param props an object containing the packet view models which contain the packet fields to make available to display
 */
const FieldsScreenContainer: Component = () => {
    return (
        // h-0 is used to make the flexbox scrollable; see https://stackoverflow.com/a/65742620/16236499 for more information
        <div class="flex flex-grow h-0">
            {/*Views*/}
            <div class="grid grid-cols-1 p-2 gap-2" style={{ "width": "100%" }}>
            
                <FieldsScreen number={1} />
            </div>
        </div>
    );
}

export default FieldsScreenContainer;
