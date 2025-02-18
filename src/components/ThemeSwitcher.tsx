import { Component, Match, Switch } from "solid-js";
import { ThemeContextValue, useTheme } from "./ThemeProvider";
import moonIcon from "../assets/moon.svg";
import sunIcon from "../assets/sun.svg";
import { WebviewWindow } from '@tauri-apps/api/window';


/**
 * A component that allows the user to toggle the theme between light and dark mode
 */
const ThemeSwitcher: Component = () => {
    const { theme, setTheme }: ThemeContextValue = useTheme();


    return (
        <div class="flex">
            <button
                class=""
                onclick={() => {
                    const id = Math.floor(Math.random() * 1000);

                    const webview = new WebviewWindow(
                        `${id}`, // Label must be unique
                        { url: `/newFlight/${id}` }
                    );
                
                    // since the webview window is created asynchronously,
                    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
                    webview.once('tauri://created', function () {
                        // webview window successfully created
                        console.log("window created");
                    })
                    webview.once('tauri://error', function (e) {
                        // an error occurred during webview window creation
                        console.log("failed to create window");
                    })
                }}>
                test123
            </button>
            <button class="p-2 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-200 border-rounded" 
                    onclick={() => setTheme(theme() === "light" ? "dark" : "light")}>
                {/* For some reason, when the theme is changed, a match is removed for long enough that the page is re-layed-out when the button does not have an icon */}
                {/* Workaround: place icons in a fixed-size container so no resize can occur */}
                <div style={{ width: "28px", height: "28px" }}>
                    <Switch>
                        <Match when={theme() === "light"}>
                            <img alt="Enable dark mode" src={moonIcon} class="w-full h-full" draggable={false} />
                        </Match>
                        <Match when={theme() === "dark"}>
                            <img alt="Enable light mode" src={sunIcon} class="w-full h-full" draggable={false} />
                        </Match>
                    </Switch>
                </div>
            </button>            
        </div>

    );
};

export default ThemeSwitcher;