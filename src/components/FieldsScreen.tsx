import { Component, For, JSX } from "solid-js";
import { useModal } from "./ModalProvider";
import ExpandedFieldsModal from "./ExpandedFieldsModal";
import upRightArrow from "../assets/up-right-arrow.png"
import broom from "../assets/broom.png"
import { createStore } from "solid-js/store";
import FieldSelectModal, { FieldSelectModalProps } from "./FieldSelectModal";

/**
 * An object that identifies a field in a packet by its packet id and field index and contains the name of the packet and field.
 */
export type FieldInPacket = {
    /**
     * The name of the packet that contains the field this `FieldInPacket` represents.
     */
    packetName: string,
    /**
     * The id of the packet that contains the field this `FieldInPacket` represents.
     */
    packetId: number,
    /**
     * The name of the field this `FieldInPacket` represents.
     */
    name: string,
    /**
     * The index of the field that this `FieldInPacket` represents inside the packet that contains the field.
     */
    fieldIndex: number,
}

/**
 * An object representing the state of a screen
 */
export type FieldsScreenState = {
    /**
     * The list of fields that can be selected
     */
    fieldsInPackets: FieldInPacket[];
    /**
     * The user-displayable number of this screen
     */
    number: number;
}

/**
 * The properties required for the {@link FieldsScreen} component.
 */
export type FieldsScreenProps = {
    /**
     * The state of the screen to display
     */
    fieldsViewState: FieldsScreenState
};

/**
 * A component that:
 * - Fisplays a list of selected fields added to this screen
 * - Allows users to add fields to the screen
 * - Allows users to clear the screen
 * - Allows users to view the graphed data received for the selected fields
 * 
 * @param props an object that contains a {@link FieldsScreenState} so that this component knows 
 */
const FieldsScreen: Component<FieldsScreenProps> = (props) => {
    const { showModal } = useModal();

    const [selected, setSelected] = createStore<FieldInPacket[]>([]);

    const handleSelect = (isChecked: boolean, packetId: number, fieldIndex: number) => {
        if (isChecked) {
            setSelected([...selected, props.fieldsViewState.fieldsInPackets.find(
                fieldInPacket => fieldInPacket.packetId === packetId && fieldInPacket.fieldIndex === fieldIndex)!]);
        } else {
            setSelected(selected.filter(
                fieldInPacket => fieldInPacket.packetId !== packetId || fieldInPacket.fieldIndex !== fieldIndex));
        }
    }

    return (
        <div class="relative bg-neutral-300 dark:bg-neutral-700 p-2">
            {/*Field Select Button*/}
            <button onClick={() => showModal<FieldSelectModalProps, {}>(FieldSelectModal, { fieldViewState: props.fieldsViewState, selectedFields: selected, handleSelect: handleSelect })}>
                Select Fields
            </button>

            {/*Expand button*/}
            <button class="absolute top-1 right-1 w-5 h-5 p-0"
                onClick={() => showModal<FieldsScreenState, {}>(ExpandedFieldsModal, { fieldsInPackets: selected, number: props.fieldsViewState.number })}>
                <img src={upRightArrow} style={{ "width": "100%", "height": "100%" }} alt="Expand"></img>
            </button>

            {/*Delete button*/}
            <button class="absolute bottom-1 right-1 w-5 h-5 p-0"
                onClick={() => { setSelected([]) }}>
                <img src={broom} style={{ "width": "100%", "height": "100%" }} alt="Delete"></img>
            </button>

            {/*Fields*/}
            <div class="absolute flex flex-wrap top-10 bottom-8 left-0 right-0 m-a p-4 items-center justify-center gap-4 overflow-y-scroll" style={{"width": "90%"}}>
                <For each={selected}>
                    {(fieldInPacket: FieldInPacket) =>
                        <div class="bg-gray p-2">
                            <h3>{fieldInPacket.packetName}</h3>
                            <p>{fieldInPacket.name}</p>
                        </div>
                    }
                </For>
            </div>
        </div>
    )
}

export default FieldsScreen;
