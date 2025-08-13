# WINGS

## Downloads

https://github.com/UMN-Rocket-Team/WINGS/releases

The Ground Station of the University of Minnesota Twin Cities Rocket Team.

## Features
 - Live telemetry reception through a serial (USB) port
 - Data visualization
 - Telemetry format customization
 - Data storage
 - Dark mode
 - Radio testing

## Running

The most-recent published executables are available in the [Releases Tab](https://github.com/UMN-Rocket-Team/WINGS/releases). Currently, CI (Continuous Integration) processes aren't set up, so any more-recent builds of the application will have to be done manually or requested from a developer with the same operating system. To manually build and run the application, follow the instructions in the [Building](#building) section.

## Developing

### Prerequisites

A [working knowledge](https://en.wiktionary.org/wiki/working_knowledge) of the following technologies will be helpful when contributing to this project:

 - [Git](https://www.git-scm.com/): source control
 - [TypeScript](https://www.typescriptlang.org/): frontend programming language
 - [`pnpm`](https://www.pnpm.io/): package management
 - [SolidJS](https://www.solidjs.com/): reactive frontend library
 - [Tailwind CSS](https://tailwindcss.com/): UI styling
 - [Tauri](https://www.tauri.app/): cross-platform window management library
 - [Rust](https://www.rust-lang.org/): backend programming language

To get started with frontend development, the [Hello World and Beyond](https://docs.google.com/document/d/19jHqrfia9sDfPGw_IvaK9lhzySdDVZC7vv07I-OWDiM/edit?usp=share_link) tutorial will be a helpful guide.

Don't read the documentation for these technologies cover to cover, instead, work at understanding what each is and be ready to ask for help, look up your questions, and reference online documentation. Examples of how to use these technologies can be found throughout this project and in online tutorials.

### Development Process

Basic familiarity with your operating system's terminal will be helpful during this process.

1. Install the recommended version of the following project dependencies:

   - [`pnpm`](https://pnpm.io/): package management
   - [Node.js](https://nodejs.org): Javascript runtime

2. Follow the instructions on Tauri's [Prerequisites page](https://tauri.app/v1/guides/getting-started/prerequisites).

3. Clone this repository, open it up in your IDE of choice, and run the following command in this project's root directory to download the frontend's dependencies:
```shell
pnpm install
```
4. To compile and run the application in development mode, execute the following in the root directory of the project:
```shell
pnpm tauri dev
```

Some changes may be automatically applied with hot module reloading while this command is running, however, others will not and will require and application restart.

## Building

Currently, builds for an operating system can only be performed on that operating system, i.e. `.exe`s can only be compiled on Windows.

1. Follow the first two steps from [Development Process](#development-process)

2. Run the following command in the root directory of the project:
   ```shell
   pnpm tauri build
   ```
3. Distribute the executable found in a subdirectory of [`src-tauri/target/release/bundle`](src-tauri/target/release/bundle)

   - Each distribution format (`.exe`, `.deb`, `.AppImage`, `.app`) will have its own directory