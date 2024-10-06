import { Component, For, JSX, createEffect, createSignal } from "solid-js";
import { BooleanStruct } from "../modals/BooleanSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import { unDecimatedPackets, parsedPackets } from "../backend_interop/buffers";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { displays } from "./DisplaySettingsScreen";

const Boolean: Component<BooleanStruct> = (boolean): JSX.Element => {
    enum Colors {
        RED = "#fc0303",
        GREY = "#6e6e6e",
        GREEN = "#1eff00"
    }
    
    const { parsedPacketCount, PacketStructureViewModels } = useBackend();

    // each index corresponds to boolean.fields value
    const [values, setValues] = createSignal([] as number[]);

    const update = () => {
        if (unDecimatedPackets[boolean.packetID] === undefined){
            setValues(boolean.fields.map(() => {
                const latestValue = 0;
                return latestValue;
            }));   
            return;     
        }
        const lastPacket = unDecimatedPackets[boolean.packetID][(unDecimatedPackets[boolean.packetID].length) -1];
        if (!lastPacket) {
            setValues([]);
            return;
        }

        setValues(boolean.fields.map(i => {
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

    const getPacket = () => PacketStructureViewModels.find(i => i.id === boolean.packetID)!;
    const getFieldComponents = () => getPacket().components.filter(i => i.type === PacketComponentType.Field);

    update();

    return <div class={`h-100% gap-2 text-center overflow-scroll overflow-x-hidden 
        ${((displays.length > 1 && boolean.fields.length < 9) || (boolean.fields.length < 17)) && 'overflow-y-hidden'}`}>

        <div class="font-bold m-b-2 text-lg">
            {boolean.displayName}

        </div>

        <div class="flex flex-wrap top-10 bottom-8 left-0 right-0 m-a p-4 items-center justify-center gap-6 content-center w-90%">
            <For each={boolean.fields}>{(item, index) => {
                const field = () => getFieldComponents()[item.packetFieldIndex].data as PacketField;

                const getValue = (): string => {
                    if (values().length <= index()) {
                        return 'N/A';
                    }
                    const value = values()[index()];
                    if (item.unit) {
                        return `${value}`;
                    }
                    return '' + value;
                };

                const getColor = (): string => {
                    let meetsCondition: boolean;
                    const value = Number(getValue());

                    if (item.isRange) {
                        if (!item.unit.left || !item.unit.right) return Colors.GREY;

                        const leftUnit = Number(item.unit.left);
                        const rightUnit = Number(item.unit.right);
                        meetsCondition = leftUnit < value && value < rightUnit;

                    } else {
                        if (!item.unit.right) return Colors.GREY;

                        const unit = Number(item.unit.right);
                        switch (item.sign) {
                            case "<":
                                meetsCondition = value < unit;
                                break;

                            case ">":
                                meetsCondition = value > unit;
                                break;

                            default:
                                meetsCondition = value === unit;
                                break;
                        }                        
                    }

                    return meetsCondition ? Colors.GREEN : Colors.RED;
                }

                return <>
                    <div class="w98px aspect-square border-rounded-xl border-0 px-4 py-2 flex flex-col justify-center align-center"
                            style={`box-shadow: 0px 0px 6px 6px ${!(getColor() === Colors.GREY) && getColor()}; 
                                background-color: ${getColor()}`}> 

                        <div>
                            <div style={{"word-wrap": "break-word"}}>{field().name}</div>
                            <div class="grow-1 max-h-120px" style={{
                                // override default macOS font with one where all the numbers are the same size
                                "font-family": '"Helvetica Neue", Helvetica, Arial, sans-serif',
                                "word-wrap": "break-word"
                            }}>
                                {getValue()}
                            </div>                                   
                        </div>
                    </div>
                </>;
            }}</For>
        </div>

    </div>
}

export default Boolean;