import { createContext, useContext, ParentComponent, createSignal, Accessor } from "solid-js";
import Theme from "../core/theme";

export type ThemeContextValue = {
    theme: Accessor<string>,
    setTheme: (theme: Theme) => void,
};

const DEFAULT_THEME: Theme = "light";

const ThemeContext = createContext<ThemeContextValue>({
    theme: () => DEFAULT_THEME,
    setTheme: () => { throw new Error("Cannot set theme in default ThemeContext implementation!"); },
});

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

export const useTheme = (): ThemeContextValue => useContext(ThemeContext);
