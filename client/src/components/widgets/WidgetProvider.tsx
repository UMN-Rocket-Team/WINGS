import React, { PropsWithChildren } from "react"
import { WidgetDefinition, WidgetPosition } from "@/components/widgets/types";

type WidgetContextValue = {
    position: WidgetPosition
    setPosition: (position: WidgetPosition) => void
}

type WidgetContextProps = {
    position: WidgetPosition
}

const widgetContextDefaultValue = {
    position: {x: NaN, y: NaN},
    setPosition: () => {},
}

export const WidgetContext = React.createContext<WidgetContextValue>(widgetContextDefaultValue);

export function WidgetProvider({children, position}: WidgetContextProps & PropsWithChildren) {
    const [_position, setPosition] = React.useState<WidgetPosition>(position)
    const Provider = WidgetContext.Provider;
    return (
        <Provider
            value={{
                position: _position,
                setPosition
            }}
        >
            {children}
        </Provider>
    )
}