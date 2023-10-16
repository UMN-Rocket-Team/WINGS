import { Component, Match, Switch } from "solid-js";
import { ThemeContextValue, useTheme } from "./ThemeProvider";
import moonIcon from "../assets/moon.svg";
import sunIcon from "../assets/sun.svg";

/**
 * A component that allows the user to toggle the theme between light and dark mode
 */
const ThemeSwitcher: Component = () => {
    const { theme, setTheme }: ThemeContextValue = useTheme();

    return (
        <button class="p-2 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-200 border-rounded" 
                onclick={() => setTheme(theme() === "light" ? "dark" : "light")}>
            {/* For some reason, when the theme is changed, a match is removed for long enough that the page is re-layed-out when the button does not have an icon */}
            {/* Workaround: place icons in a fixed-size container so no resize can occur */}
            <div style={{ width: "28px", height: "28px" }}>
                <Switch>
                    <Match when={theme() === "light"}>
                        <img alt="Enable dark mode" src={moonIcon} class="w-full h-full" />
                    </Match>
                    <Match when={theme() === "dark"}>
                        <img alt="Enable light mode" src={sunIcon} class="w-full h-full" />
                    </Match>
                </Switch>
            </div>
        </button>
    );
};

export default ThemeSwitcher;