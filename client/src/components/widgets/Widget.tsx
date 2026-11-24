import { ComponentProps, ComponentPropsWithRef, PropsWithChildren } from "react"
import { Box } from "@mui/material";
import React from "react";
import { WidgetPosition, WidgetType } from "./types";
import { WidgetProvider } from "@/components/widgets/WidgetProvider";

export type WidgetProps = {
    locked?: boolean,
    position: WidgetPosition,
    width?: number,
    height?: number,
} & ComponentPropsWithRef<typeof Box>;

export function Widget({children, className, sx, locked = false, height = 1, width = 1, position, ...rest}: WidgetProps) {
    const [_locked, setLocked] = React.useState<boolean>(locked);
    const [_position, setPosition] = React.useState<WidgetPosition>(position);
    const _sx = {
        ...sx,
        width: "100%",
        height: "100%",
        gridColumn: `${position.x} / span ${width}`,
        gridRow: `${position.y} / span ${height}`,
    };
    console.log(_sx);
    return (
        <WidgetProvider position={position}>
            <Box
                className={className}
                sx={_sx}
                {...rest}
                >
                {children}
            </Box>
        </WidgetProvider>
    )
}