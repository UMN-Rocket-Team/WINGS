import { isTauri } from "@tauri-apps/api/core"
import React, { PropsWithChildren } from "react"

type TauriContextValue = {
    isTauri: boolean,
}

export default ({children}: PropsWithChildren) => {
    const [value, _setValue] = React.useState<TauriContextValue>({isTauri: isTauri()});
    const TauriContext = React.createContext<TauriContextValue>(value);
    const Provider = TauriContext.Provider;
    return (
        <Provider
            value={value}
        >
            {children}
        </Provider>
    )
}