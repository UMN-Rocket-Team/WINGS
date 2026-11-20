import React, { PropsWithChildren } from "react";

export default function Home({children}: PropsWithChildren): React.ReactNode {
    return (
        <>
            {children}
        </>
    )
}