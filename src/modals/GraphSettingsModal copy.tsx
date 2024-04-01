import { ModalProps } from "./ModalProvider";
import DefaultModalLayout from "./DefaultModalLayout";
import { Accessor, For, JSX, createSignal } from "solid-js";
import { DisplayStruct, SettingsModalProps, displays, setDisplays } from "../components/DisplaySettingsScreen";
import { useBackend } from "../backend_interop/BackendProvider";
import { PacketComponent, PacketComponentType, PacketField, PacketStructureViewModel } from "../backend_interop/types";
import closeIcon from "../assets/close.svg";
import { produce } from "solid-js/store";

/**
 * generic interface for all g
 */
export interface VidModalProps extends SettingsModalProps{
    /** Graph that is being passed */
    displayStruct: VidStruct,
    /** Index of graph so that handleSelect[Y/X] can be called correctly! */
}
export interface VidStruct extends DisplayStruct{
}

/**
 * A modal component that allows a user to modify the fields contained in a screen.
 * 
 * @param props an object that contains a function to close the modal, the list of fields that are selected, and a callback to select a field
 */
const VidSettingsModal = (props: ModalProps<VidModalProps>): JSX.Element => {
    const { PacketStructureViewModels } = useBackend();

    /** Signal used to help handleInput revert from blank inputs to most recent name */
    const [graphCurrName, setName] = createSignal(props.displayStruct.displayName);

    /** handleInput will handle updating the graphs name and also catches blank inputs and reverts to previous name */
    const handleInput = (event: Event) => {
        const newName = (event.target as HTMLElement).textContent || '';
        if (newName.trim() !== '') {
            setGraphName(newName.trim(), props.index);
            setName(newName.trim());
        }  else {
            (event.target as HTMLElement).textContent = graphCurrName();
        }
    };

    /* handleKeyDown helps handle updating the graphName by preventing enters(newlines) */
    const handleKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'Enter') {
          event.preventDefault();
        }
    };
    
    const setGraphName = (newName: string, index: number) => {
        setDisplays( produce((s) => 
                s[index].displayName = newName))
    }

    const deleteGraph = (index: number) => {
        let newGraphs: DisplayStruct[] = [];
        for (let i = 0; i < displays.length; i++) {
            if (index !== i) {
                newGraphs.push(displays[i]);
            }
        }
        setDisplays(newGraphs);
    }
    
    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select Fields">
                    <div class='flex flex-col bg-neutral-200 dark:bg-gray p-2 rounded-10'>
                        <h3 contenteditable={true}  style="text-align:center;" class="m-2" onBlur={handleInput} onKeyDown={handleKeyDown}>
                            {graphCurrName()}
                        </h3>
                        <h3 style="text-align:center;" class="m-2">
                            Settings
                            {/* TODO!!! Allow for changing color of the graph object and variables */}
                        </h3>
                        
                        {/* Below is the set up to create a color picker for each var, in progress still. */}
                        <div class = "relative items-center justify-center" style={"text-align:center;"}>
                            <button 
                                class = " w-[10%] h-[10%] rounded-5 border-none justify-center"
                                onClick={() => {
                                    deleteGraph(props.index);
                                    props.closeModal({})
                                }}>
                                <img alt="Delete" src={closeIcon} class="w-full h-full dark:invert justify-center" draggable={false} />
                            </button>
                        </div>
                    </div>                  
        </DefaultModalLayout>
    );
};

export default VidSettingsModal;