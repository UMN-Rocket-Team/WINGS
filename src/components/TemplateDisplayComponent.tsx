import { Component, For, JSX, Show, createEffect, createSignal } from "solid-js";
import { ReadoutStruct } from "../modals/ReadoutSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import { unDecimatedPackets, parsedPackets } from "../backend_interop/buffers";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { TemplateStruct } from "../modals/TemplateSettingsModal";


/**
 * This is a very stripped down version of the Readout display, it is still functional, but without a lot fo the newer bells and whistles.
 * @param props a templateStruct
 * @returns a JSX element that displays the most recent value of the fields selected in the templateStruct
 */
const TemplateDisplayElement: Component<TemplateStruct> = (props): JSX.Element => {
    const { parsedPacketCount, PacketStructureViewModels } = useBackend();

    // each index corresponds to readout.fields value
    const [values, setValues] = createSignal([] as number[]);

    const update = () => {

        // If the packet hasn't been received set all values to Nan
        if (unDecimatedPackets[props.packetID] === undefined){
            setValues(props.fields.map(() => {
                return NaN;
            }));   
            return;     
        }
        const lastPacket = unDecimatedPackets[props.packetID][(unDecimatedPackets[props.packetID].length) -1];

        // If the last packet is Nan or undefined(which is unexpected behavior), stop looking for that packet
        if (!lastPacket) {
            setValues([]);
            return;
        }

        // Update Latest Value signal with the data from the last packet
        setValues(props.fields.map(i => {
            const latestValue = lastPacket.fieldData[i];
            return latestValue;
        }));
    };

    createEffect(() => {
        // Update this effect whenever the parsed packet count changes (meaning new
        // packets got parsed)
        const _ignored = parsedPacketCount();
        update();
    });

    const getPacket = () => PacketStructureViewModels.find(i => i.id === props.packetID)!;
    const getFieldComponents = () => getPacket().components.filter(i => i.type === PacketComponentType.Field);

    update();

    return <div class="h-100% gap-2 flex flex-col align-center justify-center text-center">
        <For each={props.fields}>{(item, index) => {
            const field = () => getFieldComponents()[item].data as PacketField;
            const getValue = (): string => {
                if (values().length <= index()) {
                    return 'N/A';
                }
                return '' + values()[index()];
            };
            return <>
                <div>{field().name}</div>
                <div class="grow-1 max-h-120px" style={{
                    // override default macOS font with one where all the numbers are the same size
                    "font-family": '"Helvetica Neue", Helvetica, Arial, sans-serif'
                }}>
                    <h1>
                        {getValue()}
                    </h1>
                </div>
            </>;
        }}</For>
    </div>
};

export default TemplateDisplayElement;
