import React, { PropsWithChildren } from "react"

type BackendContextValue = {
}

export function BackendProvider({children}: PropsWithChildren) {
    const [value, _setValue] = React.useState<BackendContextValue>({});
    const TauriContext = React.createContext<BackendContextValue>(value);
    const Provider = TauriContext.Provider;
    return (
        <Provider
            value={value}
        >
            {children}
        </Provider>
    )
}