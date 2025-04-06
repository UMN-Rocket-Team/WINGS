import { Component, For, JSX, Show, createEffect, createSignal } from "solid-js";
import { ReadoutStruct } from "../modals/ReadoutSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import { unDecimatedPackets, parsedPackets } from "../backend_interop/buffers";
import { PacketComponentType, PacketField } from "../backend_interop/types";

let _canvas: HTMLCanvasElement | null = null;
let _ctx: CanvasRenderingContext2D | null = null;

/**
 * Get a 2D rendering context. This can fail if the OS is starved!
 */
const getContext = () => {
    if (!_canvas) {
        _canvas = document.createElement('canvas');
    }
    if (!_ctx && _canvas) {
        _ctx = _canvas.getContext('2d');
    }
    return _ctx;
};

/**
 * Uses crazy web dev tricks to make an element that automatically resizes the text inside
 * to be as big as possible but not too big.
 */
const AutoAdjustFontSize = (props: {
    text: string;
}): JSX.Element => {
    const [element, setElement] = createSignal<HTMLElement>();
    const [size, setSize] = createSignal<DOMRectReadOnly>(new DOMRectReadOnly());
    const [fontFamily, setFontFamily] = createSignal('sans-serif');

    const getFontSize = (): string => {
        const maxVerticalSize = size().height;

        const ctx = getContext();
        if (!ctx) {
            // hope it fits!
            return `${maxVerticalSize}px`;
        }

        // see if this size will actually fit on screen
        ctx.font = `${maxVerticalSize}px ${fontFamily()}`;
        const measuredSize = ctx.measureText(props.text);
        if (measuredSize.width > size().width) {
            // have to scale down to fit
            const scaleFactor = size().width / measuredSize.width;
            return `${scaleFactor * maxVerticalSize}px`;
        }

        // otherwise, we know it fits
        return `${maxVerticalSize}px`;
    };

    const resizeObserver = new ResizeObserver((entries) => {
        setSize(entries[0].contentRect);
    });

    createEffect(() => {
        const el = element();
        if (!el) return;
        resizeObserver.observe(el);
        setFontFamily(getComputedStyle(el).fontFamily);
    });

    // the outer element fills the parent so we know how much space we have with resize observer
    // to avoid loops, the text is displayed in an absolutely positioned child of the outer element,
    // so that when the text is resized, it doesn't fire the observer again.
    // scary stuff!
    return <div
        ref={setElement}
        class="w-full h-full relative flex align-center justify-center text-center leading-1em"
        style={{
            "font-size": getFontSize()
        }}
    >
        {/* crazy css to center an absolute child inside of a parent element */}
        <div class="absolute top-50% left-50%" style={{
            transform: "translate(-50%, -50%)",
            "white-space": "nowrap"
        }}>
            {props.text}
        </div>
    </div>;
};

const ReadoutDisplayElement: Component<ReadoutStruct> = (readout): JSX.Element => {
    const { parsedPacketCount, PacketStructureViewModels } = useBackend();

    // each index corresponds to readout.fields value
    const [values, setValues] = createSignal([] as number[]);

    const update = () => {
        if (unDecimatedPackets[readout.packetID] === undefined){
            setValues(readout.fields.map(() => {
                const latestValue = NaN;
                return latestValue;
            }));   
            return;     
        }
        const lastPacket = unDecimatedPackets[readout.packetID][(unDecimatedPackets[readout.packetID].length) -1];
        if (!lastPacket) {
            setValues([]);
            return;
        }

        // const lastPacket = packetData[packetData.length - 1];
        setValues(readout.fields.map(i => {
            const latestValue = lastPacket.fieldData[i.packetFieldIndex];
            return latestValue;
        }));
    };

    createEffect(() => {
        // Update this effect whenever the parsed packet count changes (meaning new
        // packets got parsed)
        const _ignored = parsedPacketCount();
        update();
    });

    const getPacket = () => PacketStructureViewModels.find(i => i.id === readout.packetID)!;
    const getFieldComponents = () => getPacket().components.filter(i => i.type === PacketComponentType.Field);

    update();

    return <div class="h-100% gap-2 flex flex-col align-center justify-center text-center">
        <div class="font-bold m-b-2 text-lg">
            {readout.displayName}
        </div>

        <For each={readout.fields}>{(item, index) => {
            const field = () => getFieldComponents()[item.packetFieldIndex].data as PacketField;

            const getValue = (): string => {
                if (values().length <= index() || Number.isNaN(values()[index()])) {
                    return 'N/A'
                }
                const value = values()[index()];
                const value_string = value.toFixed(7)
                const delimited_array = value_string.split(".")
                console.log(delimited_array)
                const pre_dec = delimited_array[0].substring(delimited_array[0].length - 5,delimited_array[0].length)
                const post_dec = delimited_array[1].substring(0,3)

                const prePadding = " ".repeat(Math.max(5 - pre_dec.length,0))
                const postPadding = " ".repeat(Math.max(3 - post_dec.length,0))
                const postUnitPadding = " ".repeat(Math.max(10 - item.unit.length,0))
                if (item.unit) {
                    return prePadding + pre_dec + "." + post_dec + postPadding + " " + `${item.unit}` + postUnitPadding;
                }
                return (prePadding + pre_dec + "." + post_dec + postPadding + " ".repeat(11));
            };
            
            return <>
                <div 
                    class = "dark:text-gray-200 font-mono-Kode"
                    style={{
                        "font-size": "20px",
                    }}>
                    {field().name}
                </div>
                <div class="grow-1 max-h-120px font-mono-Kode">
                    <div
                        class="w-full h-full relative flex align-center justify-center leading-1em dark:text-gray-200"
                        style={{
                            "font-size": "30px"
                        }}
                    >
                        <div style={{
                            'white-space': "pre-wrap"
                        }}>
                            {getValue()}
                        </div>
                    </div>
                </div>
            </>;
        }}</For>
    </div>
};

export default ReadoutDisplayElement;
