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

/**
 * Used to detect the system's default color scheme and be notified when it changes.
 */
const themeSelector = window.matchMedia('(prefers-color-scheme: dark)');

/**
 * Get the Theme that matches the system's default color scheme
 */
const getOSTheme = (): Theme => themeSelector.matches ? 'dark' : 'light';

/**
 * The context that holds the global {@link ThemeContextValue}.
 */
const ThemeContext = createContext<ThemeContextValue>({
    theme: () => getOSTheme(),
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
    const [theme, setTheme] = createSignal<Theme>(getOSTheme());

    const applyTheme = () => {
        // Apply theme to the root <html> element so that dark mode applies to everything
        // including things "outside" of the page like overscroll.
        document.documentElement.classList.toggle('dark', theme() === 'dark');
        document.documentElement.classList.toggle('light', theme() === 'light');
    };

    const setThemeWrapper = (newTheme: Theme): void => {
        setTheme(newTheme);
        applyTheme();
    };
    applyTheme();

    themeSelector.addEventListener('change', () => {
        setThemeWrapper(getOSTheme());
    });

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
