import { Component, createSignal, For, ParentProps } from "solid-js";

/**
 * The properties required for the {@link TabView} component.
 */
export type TabViewProps = {
    /**
     * The list of tabs to display
     */
    tabs: Component[];
    /**
     * The list of tab names corresponding to the {@link tabs} field.
     */
    tabNames: string[];
    /**
     * The optional space-delimited set of css classes to include on the tab container
     */
    containerClasses?: string;
    /**
     * The optional space-delimited set of css classes to include on the navbar
     */
    navbarClasses?: string;
};

/**
 * A customizable component that displays a list of given tab components one a a time depending on the user's selection
 * on the tab bar.
 * 
 * @param props an object containing the tabs, the names of the tabs, and the styles of the tab container and navbar
 */
const TabView: Component<ParentProps<TabViewProps>> = (props) => {
    const [selectedTabIndex, setSelectedTabIndex] = createSignal<number>(0);

    const tabsHtml = props.tabs.map(tab => tab({}));

    return (
        <div class={`flex flex-grow flex-col p-4 gap-4 dark:bg-dark-700 ${props.containerClasses}`}>
            <nav class={`flex p-2 justify-between ${props.navbarClasses}`}>
                <div class="flex gap-2">
                    <For each={props.tabNames}>
                        {(tabName, index) => 
                            <button data-index={index()} onClick={() => setSelectedTabIndex(index())}
                                    class={`py-2 px-8 border-rounded border-0 text-base dark:text-white ${index() === selectedTabIndex() ? "bg-blue-600 text-white" : "bg-transparent"} hover:bg-blue-600 hover:text-white`}>
                                        {tabName}
                            </button>
                        }
                    </For>
                </div>
                {props.children}
            </nav>

            <div class="flex flex-grow">
                {tabsHtml[selectedTabIndex()]}
            </div>
        </div>
    );
};

export default TabView;