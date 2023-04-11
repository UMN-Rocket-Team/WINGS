import { Accessor, Component, createMemo, For } from "solid-js";
import { PacketComponentType, PacketField, PacketViewModel } from "../backend_interop/types";
import FieldsScreen, { FieldInPacket, FieldsScreenState } from "./FieldsScreen";

/**
 * The properties required for the {@link FieldsScreenContainer} component.
 */
export type FieldsScreenContainerProps = {
    /**
     * All the packet view models containing the fields to make available to display
     */
    packetViewModels: PacketViewModel[]
}

/**
 * A component that contains four {@link FieldsScreen} components arranged in a 2x2 grid.
 * 
 * @param props an object containing the packet view models which contain the packet fields to make available to display
 */
const FieldsScreenContainer: Component<FieldsScreenContainerProps> = (props: FieldsScreenContainerProps) => {
    const allFieldsInPackets: Accessor<FieldInPacket[]> = createMemo(() =>
        props.packetViewModels.map((packetViewModel: PacketViewModel) =>
            packetViewModel.components.map((component) => {
                if (component.type === PacketComponentType.Field) {
                    const data: PacketField = (component.data as PacketField);
                    return { packetName: packetViewModel.name, packetId: packetViewModel.id, name: data.name, fieldIndex: data.index };
                }
                return null;
            }).filter(packetViewModel => packetViewModel !== null) as FieldInPacket[]
        ).flat()
    );

    const viewStates: FieldsScreenState[] = [{ fieldsInPackets: allFieldsInPackets(), number: 1 }, { fieldsInPackets: allFieldsInPackets(), number: 2 }, { fieldsInPackets: allFieldsInPackets(), number: 3 }, { fieldsInPackets: allFieldsInPackets(), number: 4 }];

    return (
        // h-0 is used to make the flexbox scrollable; see https://stackoverflow.com/a/65742620/16236499 for more information
        <div class="flex flex-grow h-0">
            {/*Views*/}
            <div class="grid grid-cols-2 p-2 gap-2" style={{ "width": "100%" }}>
                <For each={viewStates}>
                    {(fieldsViewState: FieldsScreenState) =>
                        <FieldsScreen fieldsViewState={fieldsViewState} />
                    }
                </For>
            </div>
        </div>
    )
}

export default FieldsScreenContainer;
