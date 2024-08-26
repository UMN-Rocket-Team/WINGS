import { defineConfig } from 'unocss'

export default defineConfig({ 
    rules: [
      [/^drop-shadow-(\w+)$/, match => ({ "box-shadow": `0px 0px 0.5rem 0.25rem ${match[1]}` })],
    ],
    shortcuts: {
      //flexbox shortcuts
      
      //defines style for different elements across the app
      menuButton: "bg-transparent dark:border-gray-4 dark:text-white border-1 border-rounded hover:bg-blue-600 hover:text-white p-1 hover:border-transparent",
      homePageButton: "flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white py-2 px-8 border-transparent border-rounded",
      redButton: "bg-red border-rounded border-0 px-4 py-2",
      contentContainer: "border-1 p-2 border-rounded gap-2 dark:text-gray-100 dark:border-gray-4",
      inputBox: "dark:bg-dark-700 dark:border-gray-4 dark:text-white border-1 border-rounded p-1",
      listButton: "border-transparent bg-transparent hover:bg-gray hover:bg-blue-600 hover:text-white",
      WindowContainer: "bg-stone-400 dark:bg-dark-900 p-1.5 overflow-hidden rounded-7",
      //widget shortcuts are for the boxes in the packets tab that are used to show packets and packets structures
      widgetSelected:"border-transparent bg-blue-600 text-white",
      widgetNotSelected:"bg-transparent border-black dark:border-gray-4",
      widgetGeneral:"border-rounded border-1 p-2 dark:text-white",
    }
  })