import { Accessor, Component, createMemo, For } from "solid-js";
import { PacketComponentType, PacketField, PacketViewModel } from "../backend_interop/types";
import FieldsView, { FieldInPacket, FieldsViewState } from "./FieldsView";

// const samplePacketStructures: PacketStructure[] = [
//     {
//         id: 69,
//         name: "Packet 1 Name",
//         fields: [{name: "Field 1", type: 0, offsetInPacket: 0, metadataType: 0},
//             {name: "Field 2", type: 0, offsetInPacket: 0, metadataType: 0}],
//         delimiters: []
//     },
//     {
//         id: 70,
//         name: "Packet 2 Name",
//         fields: [{name: "Field 1", type: 0, offsetInPacket: 0, metadataType: 0}],
//         delimiters: []
//     },
//     {
//         id: 71,
//         name: "Packet 3 Name",
//         fields: [{name: "Field 1", type: 0, offsetInPacket: 0, metadataType: 0}],
//         delimiters: []
//     },
//     {
//         id: 72,
//         name: "Packet 4 Name",
//         fields: [{name: "Field 1", type: 0, offsetInPacket: 0, metadataType: 0}],
//         delimiters: []
//     }
// ]
//
// const sampleViewStates: FieldsViewState[] = [
//     {
//         fieldsInPackets: [
//             {packetStructure: samplePacketStructures[0], fieldIndex: 0},
//             {packetStructure: samplePacketStructures[1], fieldIndex: 0}
//         ]
//     },
//     {fieldsInPackets: []},
//     {fieldsInPackets: []}
// ]

export type FieldsPlaygroundProps = {
    packetViewModels: PacketViewModel[]
}

const FieldsPlayground: Component<FieldsPlaygroundProps> = (props: FieldsPlaygroundProps) => {
    const allFieldsInPackets: Accessor<FieldInPacket[]> = createMemo(() =>
        props.packetViewModels.map((packetViewModel: PacketViewModel) =>
            packetViewModel.components.map((component) => {
                if (component.type === PacketComponentType.Field) {
                    const data: PacketField = (component.data as PacketField);
                    return { packetName: packetViewModel.name, packetId: packetViewModel.id, name: data.name, fieldIndex: data.index };
                }
                return null;
            }).filter(packetViewModel => packetViewModel !== null) as FieldInPacket[]
        ).flat());

    const viewStates: FieldsViewState[] = [{ fieldsInPackets: allFieldsInPackets() }, { fieldsInPackets: allFieldsInPackets() }, { fieldsInPackets: allFieldsInPackets() }, { fieldsInPackets: allFieldsInPackets() }];

    return (
        // h-0 is used to make the flexbox scrollable; see https://stackoverflow.com/a/65742620/16236499 for more information
        <div class="flex flex-grow h-0">
            {/*Views*/}
            <div class="grid grid-cols-2 p-2 gap-2 bg-red-7" style={{ "width": "100%" }}>
                <For each={viewStates}>
                    {(fieldsViewState: FieldsViewState) =>
                        <FieldsView fieldsViewState={fieldsViewState}></FieldsView>
                    }
                </For>
            </div>
        </div>
    )
}

export default FieldsPlayground;
