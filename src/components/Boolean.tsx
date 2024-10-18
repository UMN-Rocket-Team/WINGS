import { Component, For, JSX, createEffect, createSignal, createMemo } from "solid-js";
import { BooleanStruct } from "../modals/BooleanSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import { unDecimatedPackets, parsedPackets } from "../backend_interop/buffers";
import { PacketComponentType, PacketField, PacketComponent } from "../backend_interop/types";
import { displays, setDisplays } from "./DisplaySettingsScreen";
import { store } from "../core/file_handling";
import { produce } from "solid-js/store";
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

    return <div class={`h-100% gap-2 text-center overflow-scroll overflow-x-hidden 
        ${((displays.length > 1 && boolean.fields.length < 9) || (boolean.fields.length < 17)) && 'overflow-y-hidden'}`}>

        <div class="font-bold m-b-2 text-lg">
            {boolean.displayName}

        </div>

        <div class="flex flex-wrap top-10 bottom-8 left-0 right-0 m-a p-4 items-center justify-center gap-6 content-center w-90%">
            <For each={boolean.fields}>{(item, index) => {
                const packetComponent = getFieldComponents(item.packetID)[item.packetFieldIndex];
                const field = () => packetComponent.data as PacketField;
                console.log("id: ", item.packetID, "idx: ", item.packetFieldIndex);
                // setSelectedPacketStructureID(item.packetID);
                // setSelectedPacketComponentIndex(item.packetFieldIndex);
                const [packetIDAccessor, _] = createSignal<number>(item.packetID);
                const [packetComponentAccessor, setPacketComponentAccessor] = createSignal<PacketComponent>(packetComponent);
                const invokeApiSetter = createInvokeApiSetterFunction(packetIDAccessor, packetComponentAccessor, showModal);
                
                // console.log(`selected --- ${selectedPacketStructureID()} index: ${selectedPacketComponentIndex()}`)
                
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
                            {/* <div style={{"word-wrap": "break-word"}}>{field().name}</div> */}
                            <div style={{"word-wrap": "break-word"}}>
                                <input 
                                    value={field().name}
                                    onInput={async (e) => {
                                        await invokeApiSetter(setFieldName, (e.target as HTMLInputElement).value)
                                    }}
                                />
                            </div>
                            {/* <h2>
                                <input
                                    value={props.displayStruct.displayName}
                                    class="text-lg border-0 p-0 m-0 bg-transparent text-center font-bold"
                                    onChange={(e) => {
                                        console.log("sdfsdf")
                                        setDisplays(produce(s => {
                                            const struct = s[props.index] as BooleanStruct;
                                            const value = (e.target as HTMLInputElement).value.trim();
                                            if (value) {
                                                struct.displayName = value;
                                                oldName = value;
                                            } else {
                                                struct.displayName = oldName;
                                            }
                                        }));
                                        store.set("display", displays);
                                    }}
                                />
                            </h2>                             */}

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