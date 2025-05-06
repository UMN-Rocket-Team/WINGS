import { ModalProps } from "../core/ModalProvider";
import DefaultModalLayout from "../core/DefaultModalLayout";
import { For, JSX, Show, createSignal, onMount } from "solid-js";
import {SettingsModalProps, displays, setDisplays } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponentType, PacketField, PacketStructureViewModel } from "../backend_interop/types";
import { createStore, produce } from "solid-js/store";
import settingsIcon from "../assets/settings.png";
import infoIcon from "../assets/info-sym.svg";
import dropdownIcon from "../assets/dropdown.svg";
import { DisplayStruct } from "../core/display_registry";
import { ROCKET_MODELS } from "../components/Rocket";

export class RocketStruct implements DisplayStruct {
    // Implementing required values
    displayName = 'Rocket';
    packetID = -1;
    type = 'rocket';

    // unused by RocketStruct
    packetsDisplayed: boolean[] = [false];

    fieldRoll: number = -1;
    fieldPitch: number = -1;
    fieldYaw: number = -1;

    // see ROCKET_MODELS in Rocket.tsx
    rocketModel: string = 'thomas-weber-gopher';
}

const FieldList = (props: {
    packet: PacketStructureViewModel,
    selectedField: number,
    onSelectedField: (newField: number) => void
}) => {
    return (
        <select
            value={props.selectedField}
            onChange={(e) => {
                props.onSelectedField(+e.target.value);
            }}
        >
            <option value={-1}>
                (None)
            </option>
            <For
                each={props.packet.components.filter(i => i.type === PacketComponentType.Field)}
            >{(component, componentIndex) => {
                const field = component.data as PacketField;
                return (
                    <option value={field.index}>
                        {field.name}
                    </option>
                )
            }}</For>
        </select>
    );
};

// The modal that will be displayed to the user when editing a template type
const RocketSettingsModal = (props: ModalProps<SettingsModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

    // Used to restore previous name when user enters something invalid
    let oldName = props.displayStruct.displayName;

    const [displaySettings, setDisplaySettings] = createSignal(false); // Are the modal settings (on the top left of the modal) being displayed?
    const [displayInfo, setDisplayInfo] = createSignal(false); // Is info about the display being displayed?

    const [displayStruct, setDisplayStruct] = createStore(props.displayStruct as RocketStruct);

    let infoIconRef: HTMLImageElement | undefined;
    onMount(() => { // Events for hovering over info icon
        infoIconRef?.addEventListener("mouseout", (e) => {
            setDisplayInfo(false);
        });
        infoIconRef?.addEventListener("mouseover", (e) => {
            setDisplayInfo(true);
        });
    });

    /** HandleInput will handle updating the graphs name and also catches blank inputs and reverts to previous name */
    const handleInput = (event: Event) => {
        const newName = (event.target as HTMLElement).textContent || '';
        if (newName.trim() !== '') {
            setDisplayName(newName.trim(), props.index);
            oldName = newName.trim();
        } else {
            (event.target as HTMLElement).textContent = oldName;
        }
    };

    /* HandleKeyDown helps handle updating the graphName by preventing enters(newlines) */
    const handleKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'Enter') {
            event.preventDefault();
        }
    };

    const setDisplayName = (newName: string, index: number) => {
        setDisplays(produce(s => {
            s[index]!.displayName = newName;
        }));
    };

    const deleteDisplay = () => {
        setDisplays(displays.filter((_, index) => index !== props.index));
        props.closeModal({});
    };

    const getPacket = () => {
        return PacketStructureViewModels.find(i => i.id === displayStruct.packetID);
    };

    return <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
        <div class="flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg relative min-w-fit">

            {/*More info button*/}
            <Show when={displayInfo()}>
                <div class="absolute bg-neutral-300 top-[-1px] left-[-1px] dark:bg-neutral-700 p-4 rounded-3xl pt-12 z-[2]">
                    <p class="max-w-prose">Determine which packets will affect how the rocket is rendered</p>
                </div>
            </Show>

            {/*name of the display, along with the Extra settings button*/}
            <div class='flex flex-row leading-none justify-between mb-4'>
                <img alt="Info" src={infoIcon} ref={infoIconRef} draggable={false} class="relative top-0 w-[23px] dark:invert z-[3]" />

                <h3 contenteditable={true} class="m-2 text-center font-bold w-[82%] absolute left-[50%] translate-x-[-50%]"
                    onBlur={handleInput} onKeyDown={handleKeyDown}>
                    {props.displayStruct.displayName}
                </h3>

                <img alt="Settings" src={settingsIcon} draggable={false} onClick={() => setDisplaySettings(s => !s)}
                    class="relative top-0 w-[25px] dark:invert z-[1] cursor-pointer" />
            </div>

            {/*
            Extra settings, which always contain the button to delete the display,
            they can also contain other settings that apply to the entire modal, like toggling a debug mode and
            */}
            <Show when={displaySettings()}>
                <div class="absolute bg-neutral-300 dark:bg-neutral-700 p-4 top-0 rounded-3xl right-0 z-[0]">
                    <div class="relative flex items-center justify-center mt-10">
                        <button
                            class="rounded-lg bg-red-500 hover:bg-red-600 flex items-center justify-center p-3"
                            onClick={() => {
                                deleteDisplay();
                            }}>
                            <h3>Remove Display</h3>
                        </button>
                    </div>
                </div>
            </Show>

            <p>Select rocket model:</p>
            <select
                value={displayStruct.rocketModel}
                onChange={(e) => {
                    setDisplays(produce(s => {
                        const struct = (s[props.index] as RocketStruct);
                        struct.rocketModel = e.target.value;
                    }));
                }}
            >
                <For each={Object.keys(ROCKET_MODELS)}>{modelName => (
                    <option value={modelName}>
                        {modelName}
                    </option>
                )}</For>
            </select>

            <p>Select which packet to use:</p>
            <select
                value={displayStruct.packetID}
                onChange={(e) => {
                    setDisplays(produce(s => {
                        const struct = (s[props.index] as RocketStruct);
                        struct.packetID = +e.target.value;
                        struct.fieldRoll = -1;
                        struct.fieldPitch = -1;
                        struct.fieldYaw = -1;
                    }));
                }}
            >
                <option value={-1}>
                    (None)
                </option>
                <For each={PacketStructureViewModels}>{(packetViewModel, packetIdx) => (
                    <option value={packetViewModel.id}>
                        {packetViewModel.name}
                    </option>
                )}</For>
            </select>

            <Show when={displayStruct.packetID !== -1}>
                <p>Roll field:</p>
                <FieldList
                    packet={getPacket()!}
                    selectedField={displayStruct.fieldRoll}
                    onSelectedField={(newField) => {
                        setDisplays(produce(s => {
                            const struct = (s[props.index] as RocketStruct);
                            struct.fieldRoll = newField;
                        }));
                    }}
                />

                <p>Pitch field:</p>
                <FieldList
                    packet={getPacket()!}
                    selectedField={displayStruct.fieldPitch}
                    onSelectedField={(newField) => {
                        setDisplays(produce(s => {
                            const struct = (s[props.index] as RocketStruct);
                            struct.fieldPitch = newField;
                        }));
                    }}
                />

                <p>Yaw field:</p>
                <FieldList
                    packet={getPacket()!}
                    selectedField={displayStruct.fieldYaw}
                    onSelectedField={(newField) => {
                        setDisplays(produce(s => {
                            const struct = (s[props.index] as RocketStruct);
                            struct.fieldYaw = newField;
                        }));
                    }}
                />
            </Show>
        </div>
    </DefaultModalLayout>;
};

export default RocketSettingsModal;