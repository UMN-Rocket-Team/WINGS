# Downloads

https://github.umn.edu/Rocket-Team/wings/releases

# links to external materials

new member training:
https://github.umn.edu/Rocket-Team/UFC-2/wiki/Software-Training

coding practices:
https://github.umn.edu/Rocket-Team/UFC-2/wiki/Software-Coding-Practices

Wiki:
https://github.umn.edu/Rocket-Team/UFC-2/wiki/Wings-Reference

# old README below

# wings

The Ground Station of the University of Minnesota Twin Cities Rocket Team.

## Features
 - Live telemetry reception through a serial (USB) port
 - Data visualization
 - Telemetry format customization
 - Data storage
 - Dark mode
 - Radio testing

## Running

The most-recent published executables are available in the [Releases Tab](https://github.umn.edu/Rocket-Team/wings/releases). Currently, CI (Continuous Integration) processes are not enabled on the University of Minnesota's GitHub instance, so any more-recent builds of the application will have to be done manually or requested from a developer with the same operating system. To manually build and run the application, follow the instructions in the [Building](#building) section.

## Building

Basic familiarity with your operating system's terminal will be helpful during this process.

Currently, builds for an operating system can only be performed on that operating system, i.e. `.exe`s can only be compiled on Windows.

1. Install the recommended version of the following project dependencies:

   - [`pnpm`](https://pnpm.io/): package management
   - [Node.js](https://nodejs.org): Javascript runtime

2. Follow the instructions on Tauri's [Prerequisites page](https://tauri.app/v1/guides/getting-started/prerequisites).
3. Run the following command in the root directory of the project:
   ```shell
   pnpm tauri build
   ```
4. Distribute the executable found in a subdirectory of [`src-tauri/target/release/bundle`](src-tauri/target/release/bundle)

   - Each distribution format (`.exe`, `.deb`, `.AppImage`, `.app`) will have its own directory

### Building on Linux
1. Follow the insturctions to install pnpm and Node.js. Ensure that you install npm as you install Node.js. Your Node.js version must be over v12.
   - [`pnpm`](https://pnpm.io/): package management
   - [Node.js]([https://nodejs.org](https://nodejs.org/en/download/package-manager)): Javascript runtime
 2. Run `npm install` in the

## Developing

### Prerequisites

A [working knowledge](https://en.wiktionary.org/wiki/working_knowledge) of the following technologies will be helpful when contributing to this project:

 - [Git](https://www.git-scm.com/): source control
 - [Typescript](https://www.typescriptlang.org/): frontend programming language
 - [`pnpm`](https://www.pnpm.io/): package management
 - [SolidJS](https://www.solidjs.com/): reactive frontend library
 - [UnoCSS](https://github.com/unocss/unocss): UI styling
 - [Tauri](https://www.tauri.app/): cross-platform window management library
 - [Rust](https://www.rust-lang.org/): backend programming language

To get started with frontend development, the [Hello World and Beyond](https://docs.google.com/document/d/19jHqrfia9sDfPGw_IvaK9lhzySdDVZC7vv07I-OWDiM/edit?usp=share_link) tutorial will be a helpful guide.

Don't read the documentation for these technologies cover to cover, instead, work at understanding what each is and be ready to ask for help, look up your questions, and reference online documentation. Examples of how to use these technologies can be found throughout this project and in online tutorials.

### Development Process

This project uses a feature-driven development process. Work on a specific feature or issue is done on its own Git branch, then merged into the `main` branch when it is complete.

To start work on a feature, create a new branch named specifically like `feature-new-packetexplorer-ui` based on the `main` branch. If the feature is large, try to separate related code into their own incremental commits. Code divided logically into smaller chunks is much easier to review, especially if the program still functions after the commit. Once the feature is finished, push your code to the repository and make a pull request into the `main` branch for review, which is required.

To keep others up to date on what features are being worked on, either assign yourself to the GitHub issue for the feature or create a new one.

The same process applies for bugfixes.

### Contributing

To get started contributing, follow the first two steps in the [Building](#building) section, clone this repository, open it up in your IDE of choice, and run the following command in this project's root directory to download the frontend's dependencies:
```shell
pnpm install
```

Then, follow the process outlined above.

To compile and run the application in development mode, execute the following in the root directory of the project:
```shell
pnpm tauri dev
```

Some changes may be automatically applied with hot module reloading while this command is running, however, others will not and will require and application restart.
