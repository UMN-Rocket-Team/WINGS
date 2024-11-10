import { Component, For, JSX, createEffect, createSignal, Show, onMount } from "solid-js";
import { BooleanStruct } from "../modals/BooleanSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import { unDecimatedPackets } from "../backend_interop/buffers";
import { PacketComponentType, PacketField, PacketComponent } from "../backend_interop/types";
import { displays, setDisplays } from "./DisplaySettingsScreen";
import { setFieldName } from "../backend_interop/api_calls";
import { createInvokeApiSetterFunction } from "../core/packet_editor_helpers";
import { useModal } from "../modals/ModalProvider";

const Boolean: Component<BooleanStruct> = (boolean): JSX.Element => {
    enum Colors {
        RED = "#fc0303",
        GREY = "#6e6e6e",
        GREEN = "#1eff00"
    }

    const { showModal } = useModal();

    let textAreaRef: HTMLTextAreaElement | undefined;
    onMount(() => {if (textAreaRef) textAreaRef.style.height = textAreaRef?.scrollHeight + "px"});
    
    const { parsedPacketCount, PacketStructureViewModels } = useBackend();

    // each index corresponds to boolean.fields value
    const [values, setValues] = createSignal([] as number[]);

    const update = () => {
        const updatePacket = (packetID: number) => {
            if (unDecimatedPackets[packetID] === undefined){
                setValues((v) => { // Setting all values in packet of packetID to 0
                    let newVals = boolean.fields.map((field, i) =>
                        field.packetID === packetID ? 0 : v[i]
                    );
                    return newVals;
                });   
                return;     
            }
            
            const lastPacket = unDecimatedPackets[packetID][(unDecimatedPackets[packetID].length) -1];
            if (!lastPacket) {
                setValues((v) => { // Removing all values in packet of packetID
                    let newVals: number[] = [];
                    for (let i=0; i<boolean.fields.length; i++) {
                        const field = boolean.fields[i];
                        if (field.packetID !== packetID) newVals.push(v[i]);
                    }
        
                    return newVals;
                });   
                return;  
            }

            setValues((v) => { // Changing all values in packet of packetID to values from lastPacket
                let newVals = boolean.fields.map((field, i) =>
                    field.packetID === packetID ? lastPacket.fieldData[field.packetFieldIndex] : v[i]
                );
                return newVals;
            });
        }

        let updatedPackets: {[id: number]: boolean} = {};
        boolean.fields.forEach((f) => {
            if (!updatedPackets[f.packetID]) {
                updatePacket(f.packetID);
                updatedPackets[f.packetID] = true;
            }
        });
    };

    createEffect(() => {
        // Update this effect whenever the parsed packet count changes (meaning new
        // packets got parsed)
        const _ignored = parsedPacketCount();
        update();
    });

    const getPacket = (packetID: number) => {
        return PacketStructureViewModels.find(i => i.id === packetID)!;
    }
    const getFieldComponents = (packetID: number) => getPacket(packetID).components.filter(i => i.type === PacketComponentType.Field);

    update();

    return <div class={`h-full gap-2 text-center overflow-y-auto overflow-x-hidden `}>

        <div class="font-bold mb-2 text-lg">
            {boolean.displayName}

        </div>

        <div class="flex flex-wrap top-10 bottom-8 left-0 right-0 m-auto p-4 items-center justify-center gap-6 content-center w-9/10">
            <For each={boolean.fields}>{(item, index) => {
                const packetComponent = getFieldComponents(item.packetID)[item.packetFieldIndex];
                const field = () => packetComponent.data as PacketField;

                const [packetIDAccessor, _] = createSignal<number>(item.packetID);
                const [packetComponentAccessor, setPacketComponentAccessor] = createSignal<PacketComponent>(packetComponent);
                const invokeApiSetter = createInvokeApiSetterFunction(packetIDAccessor, packetComponentAccessor, showModal);

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
                    <div class="w-[112px] aspect-square rounded-xl border-0 px-4 py-2 flex flex-col justify-center items-center max-h-[112px]"
                            style={`box-shadow: 0px 0px 6px 6px ${!(getColor() === Colors.GREY) && getColor()}; 
                                background-color: ${getColor()}`}> 
                        <div>
                            <textarea class="border-0 bg-transparent p-0 m-0 text-center w-full resize-none max-h-[5em] overflow-y-hidden" 
                                style={
                                    `word-wrap: break-word; 
                                    word-break: break-all; 
                                    scrollbar-width: thin;
                                    font-family: inherit;
                                    font-size: inherit;`
                                }
                                rows="1"
                                ref={textAreaRef}
                                spellcheck={false}
                                
                                onInput={async (e) => {
                                    e.target.style.height = 'auto';
                                    e.target.style.height = e.target.scrollHeight + "px";

                                    const content: string = (e.target as HTMLTextAreaElement).value || "";
                                    await invokeApiSetter(setFieldName, content);
                                }}
                            >{field().name}</textarea>

                            <div class="grow max-h-[120px]" style={{
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