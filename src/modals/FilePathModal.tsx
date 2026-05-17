import { ModalProps } from "@/core/ModalProvider";
import DefaultModalLayout from "@/core/DefaultModalLayout";
import { For, JSX, createSignal, Show } from "solid-js";
import { ImportWindowOptions, runImportPacketWindow } from "@/core/file_handling";
import { ProductName } from "@/backend_interop/types";
import { addCsvFile, addBinaryFile } from "@/backend_interop/api_calls";

/**
 * The properties required for the {@link FileModal} component.
 */
export type FileModalProps = {
    /** Optional configuration for the file-picker dialog (filters, title, etc). */
    importWindowOptions?: ImportWindowOptions;
};

type FileType = "binary" | "csv" | null;

/**
 * A modal component that allows users to choose between binary (.wings) and CSV files,
 * and for binary files, select the ProductName that the flight data came from.
 * 
 * @param props an object that contains a function to close the modal and the error message and description
 */
const FileModal = (props: ModalProps<FileModalProps>): JSX.Element => {
    const [selectedFileType, setSelectedFileType] = createSignal<FileType>(null);
    const [selectedBinaryPath, setSelectedBinaryPath] = createSignal<string | null>(null);

    const productNames: ProductName[] = ["altusMetrum", "rfd", "featherweight", "aim", "midwest"];

    const getSinglePath = (filePaths: string | string[] | null) => {
        if (Array.isArray(filePaths)) {
            return filePaths[0] ?? null;
        }
        return filePaths;
    };

    const handleCsvFile = async () => {
        const filePath = getSinglePath(await runImportPacketWindow({
            title: "Select CSV File",
            multiple: false,
            filterName: "CSV Files",
            extensions: ["csv"]
        }));

        if (filePath) {
            await addCsvFile(filePath);
            props.closeModal({});
        }
    };

    const handleBinaryFileSelection = async () => {
        const filePath = getSinglePath(await runImportPacketWindow({
            title: "Select Binary File",
            multiple: false,
            filterName: "Binary Files",
            extensions: ["wings", "bin"]
        }));

        if (filePath) {
            setSelectedBinaryPath(filePath);
        }
    };

    const handleProductNameSelection = async (productName: ProductName) => {
        const filePath = selectedBinaryPath();
        if (filePath) {
            await addBinaryFile(filePath, productName);
            props.closeModal({});
        }
    };

    return (
        <DefaultModalLayout close={() => props.closeModal({})} title="Select File Type">
            <Show
                when={selectedFileType() === null}
                fallback={
                    <Show
                        when={selectedFileType() === "binary" && selectedBinaryPath() !== null}
                        fallback={
                            <div class="flex flex-col gap-3">
                                <button
                                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-full"
                                    onClick={handleBinaryFileSelection}
                                >
                                    Select Binary File
                                </button>
                                <button
                                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded w-full"
                                    onClick={() => setSelectedFileType(null)}
                                >
                                    Back
                                </button>
                            </div>
                        }
                    >
                        <div class="flex flex-col gap-3">
                            <p class="text-sm font-semibold mb-2">Selected: {selectedBinaryPath()}</p>
                            <p class="text-sm font-semibold mb-2">Select the product that the flight data came from:</p>
                            <For each={productNames}>
                                {(productName) => (
                                    <button
                                        class="bg-green-500 hover:bg-green-700 text-white font-semibold py-2 px-4 rounded w-full text-left"
                                        onClick={() => handleProductNameSelection(productName)}
                                    >
                                        {productName}
                                    </button>
                                )}
                            </For>
                            <button
                                class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded w-full"
                                onClick={() => {
                                    setSelectedFileType(null);
                                    setSelectedBinaryPath(null);
                                }}
                            >
                                Back
                            </button>
                        </div>
                    </Show>
                }
            >
                <div class="flex flex-col gap-3">
                    <p class="text-sm font-semibold mb-2">Choose file type:</p>
                    <button
                        class="bg-purple-500 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded w-full"
                        onClick={() => setSelectedFileType("binary")}
                    >
                        Binary File (.wings)
                    </button>
                    <button
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-full"
                        onClick={handleCsvFile}
                    >
                        CSV File
                    </button>
                </div>
            </Show>
        </DefaultModalLayout>
    );
};

export default FileModal;
