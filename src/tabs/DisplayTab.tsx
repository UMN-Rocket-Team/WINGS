import { Component, For, JSX, Show, Switch, Match } from "solid-js";
import { displays, FlexviewObject, flexviewObjects } from "../components/DisplaySettingsScreen";
import { displayRegistry} from "../core/display_registry";

const RecursiveFlexviewViewer = (props: { object: FlexviewObject }) => {
  return (
    <Switch fallback={<div>Unknown flexview object</div>}>

      <Match when={props.object!.type === "display" ? props.object : null}>
        {(disp) => {
          const display = disp();

          const typeDef = displayRegistry.get(displays[display.index]!.type)!;
          const DisplayComponent = typeDef.displayComponent;

          return (
            <div class="overflow-hidden w-full h-full flex flex-shrink items-center justify-center border-2 border-gray-700 dark:border-gray-300 p-2">
              <DisplayComponent {...displays[display.index]!} />
            </div>
          );
        }}
      </Match>

      <Match when={props.object!.type === "layout" ? props.object : null}>
        {(layoutObj) => {
          const layout = layoutObj();

          const totalWeight = () =>
            layout.weights.reduce((a, b) => a + b, 0);

          return (
            <div
              class="overflow-hidden w-full h-full flex items-stretch justify-center border-2 border-gray-400 dark:border-gray-600 p-2 gap-2"
              style={{ "flex-direction": layout.direction }}
            >
              <Show when={layout.children.length > 0} fallback={<p>Empty layout</p>}>
                <For each={layout.children}>
                  {(childId, i) => {
                    const weight = () =>
                      `${(layout.weights[i()] / totalWeight()) * 100}%`;

                    return (
                      <div
                        style={
                          layout.direction === "column"
                            ? { height: weight() }
                            : { width: weight() }
                        }
                      >
                        <RecursiveFlexviewViewer
                          object={flexviewObjects[childId]}
                        />
                      </div>
                    );
                  }}
                </For>
              </Show>
            </div>
          );
        }}
      </Match>

    </Switch>
  );
};


const DisplayTab: Component = (): JSX.Element => {
    return (
        <div class="flex flex-col flex-grow flex-shrink gap-4 rounded-lg dark:text-white">
            {/* Views */}
            {/* <div class="grid gap-2 h-full" style={{ "grid-auto-rows": "1fr", "grid-template-columns": `repeat(${Math.min(2, displays.length)}, 1fr)` }}>
                <For each={displays}>
                    {(display: DisplayStruct) => {
                        const typeDef = displayRegistry.get(display.type)!;
                        const DisplayComponent = typeDef?.displayComponent;
                        
                        return (
                        <div class="relative" style={{ height: '40vh' }}>
                            <DisplayComponent {...display} />
                        </div>
                        );
                    }}
                </For>
            </div> */}
            <RecursiveFlexviewViewer
                object={flexviewObjects[0]}
            />
        </div>
    );
};

export default DisplayTab;
