import { Component, createSignal, For, ParentProps } from "solid-js";

export type TabViewProps = {
    tabs: Component[];
    tabNames: string[];
    containerClasses?: string;
    navbarClasses?: string;
};

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