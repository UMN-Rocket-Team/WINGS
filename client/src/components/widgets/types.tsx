import { NewWidgetWidget } from "./NewWidgetWidget"

export enum WidgetType {
    NewWidgetWidget,
}

export type WidgetPosition = {
    x: number,
    y: number
}

export type WidgetDefinition = {
    type: WidgetType,
    position: WidgetPosition,
    locked: boolean
}

export function WidgetBuilder(type: WidgetType, position: WidgetPosition) {
    
}