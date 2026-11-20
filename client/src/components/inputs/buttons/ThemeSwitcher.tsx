import { ThemeContextValue, useTheme } from "@/context/ThemeProvider";
import moonIcon from "@/assets/moon.svg";
import sunIcon from "@/assets/sun.svg";

/**
 * A component that allows the user to toggle the theme between light and dark mode
 */
const ThemeSwitcher = () => {
    const { theme, setTheme }: ThemeContextValue = useTheme();

    return (
        <button className="p-2 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-200 border-rounded" 
                onClick={() => setTheme(theme === "light" ? "dark" : "light")}>
            {/* For some reason, when the theme is changed, a match is removed for long enough that the page is re-layed-out when the button does not have an icon */}
            {/* Workaround: place icons in a fixed-size container so no resize can occur */}
            <div style={{ width: "28px", height: "28px" }}>
                {theme === 'light'
                ? <img alt="Enable dark mode" src={moonIcon} className="w-full h-full" draggable={false} />
                : <img alt="Enable light mode" src={sunIcon} className="w-full h-full" draggable={false} />
                }
            </div>
        </button>
    );
};

export default ThemeSwitcher;