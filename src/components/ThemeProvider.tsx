import { createContext, useContext, ParentComponent, createSignal, Accessor } from "solid-js";
import Theme from "../core/theme";

/**
 * The global state managed by the {@link ThemeContext}.
 */
export type ThemeContextValue = {
    /**
     * An accessor to get the current theme
     */
    theme: Accessor<Theme>,
    /**
     * Sets the current theme to the new given value
     * 
     * @param theme the new theme
     */
    setTheme: (theme: Theme) => void,
};

const DEFAULT_THEME: Theme = "light";

/**
 * The context that holds the global {@link ThemeContextValue}.
 */
const ThemeContext = createContext<ThemeContextValue>({
    theme: () => DEFAULT_THEME,
    setTheme: () => { throw new Error("Cannot set theme in default ThemeContext implementation!"); },
});

/**
 * A component that manages a global theme state and interactions with it.
 * 
 * @param props the children to provide a global theme state to
 * @returns a component wrapping the given child component that provides a global theme
 * @see {@link BackendContextValue} for the provided global functions to access and change the theme
 */
export const ThemeProvider: ParentComponent = (props) => {
    const [theme, setTheme] = createSignal<Theme>(DEFAULT_THEME);

    const setThemeWrapper = (newTheme: Theme): void => {
        document.body.classList.remove(theme());
        document.body.classList.add(newTheme);
        setTheme(newTheme);
    };

    const context = { theme, setTheme: setThemeWrapper };

    return (
        <ThemeContext.Provider value={context}>
            {props.children}
        </ThemeContext.Provider>
    );
};

/**
 * Use the theme state provided by the {@link ThemeContext}.
 * 
 * @returns the current {@link ThemeContextValue}
 */
export const useTheme = (): ThemeContextValue => useContext(ThemeContext);
