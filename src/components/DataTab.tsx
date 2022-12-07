import {Component, createSignal, For} from "solid-js";
// import {BackendInteropManagerContextValue, useBackendInteropManager} from "./BackendInteropManagerProvider";
import {PacketStructure, SerialPortNames} from "../backend_interop/types";
import BroadcastModal from "./BroadcastModal";
import {useModal} from "./ModalProvider";
import PacketTab from "./PacketTab";
// import GraphScreen from "./GraphScreen";
// import ExpandedFieldsModal from "./ExpandedFieldsModal";
import FieldsBox, {FieldsBoxProps} from "./FieldsBox";
import {createStore} from "solid-js/store";

const DataTab: Component = () => {
    const { showModal } = useModal();
    // const connectionState = createSignal(true);

    // const { availablePortNames }: BackendInteropManagerContextValue = useBackendInteropManager();
    const sampleSerialPortNames: SerialPortNames[] = [
        {name: "Sample COM 1", manufacturer_name: "Sample Manufacturer 1", product_name: "Sample Product 1"},
        {name: "Sample COM 2", manufacturer_name: "Sample Manufacturer 2", product_name: "Sample Product 2"}
    ];

    const samplePacketStructures: PacketStructure[] = [
        {
            id: 69,
            name: "Packet 1 Name",
            fields: [{name: "Field 1", type: 0, offsetInPacket: 0, metadataType: 0},
                {name: "Field 2", type: 0, offsetInPacket: 0, metadataType: 0}],
            delimiters: []
        },
        {
            id: 70,
            name: "Packet 2 Name",
            fields: [{name: "Field 1", type: 0, offsetInPacket: 0, metadataType: 0}],
            delimiters: []
        }
    ]

    const sampleFieldBoxes: FieldsBoxProps[] = [
        {
            fieldsInPackets: [
                {packetStructure: samplePacketStructures[0], fieldIndex: 0},
                {packetStructure: samplePacketStructures[1], fieldIndex: 0}
            ]
        }
    ]

    // for sample testing
    const [fieldBoxes, setFieldBoxes] = createStore<FieldsBoxProps[]>(sampleFieldBoxes);

    return (
        <div class="flex flex-col flex-grow gap-4 dark:text-white">
            <div class="flex flex-grow">
                {/*packets and fields list*/}
                <div class="flex flex-col p-2 gap-2 bg-yellow">
                    <p>Packets</p>
                    <For each={samplePacketStructures}>
                        {(packet: PacketStructure) =>
                            <PacketTab name={packet.name} fields={packet.fields} id={packet.id} delimiters={packet.delimiters}></PacketTab>
                        }
                    </For>
                </div>

                <div class="flex flex-col p-2 gap-2 bg-red-7">
                    <For each={fieldBoxes}>
                        {(fieldsBoxProps: FieldsBoxProps) =>
                            <FieldsBox fieldsInPackets={fieldsBoxProps.fieldsInPackets}></FieldsBox>
                        }
                    </For>

                    {/*add box button*/}
                    <button class="p-2" onClick={() => setFieldBoxes([...fieldBoxes, sampleFieldBoxes[0]])}>+</button>
                </div>
            </div>

            {/*bottom bar*/}
            <footer class="flex p-2 bg-gray">
                <div class="flex w-2xl">
                    <p>[Back button icon]</p>
                    <p class="dark:text-white">Serial Port:</p>
                    <input list="dataSerialPorts" name="Serial Port :"/>
                    <datalist id="dataSerialPorts">
                        <For each={sampleSerialPortNames}>
                            {(serialPort) => <option value={serialPort.name} /> }
                        </For>
                    </datalist>

                    <button>Connect/Disconnect</button>
                </div>

                <div class="flex w-2xl">
                    <p>Packets Received: [# Packets Received]</p>
                </div>

                <div class="flex w-2xl">
                    <button onClick={() => showModal<{}, {}>(BroadcastModal, {})}>Broadcast</button>
                    <button>Save</button>
                    <button>Upload</button>
                </div>
            </footer>
        </div>
    );
};

export default DataTab;