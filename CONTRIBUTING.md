# Contributing to WINGS

We welcome contributions from UMN rocket team members, other students, alumni, and the public.

## Development Process

Our workflow:

1. **Fork** this repository to your own GitHub account.
2. **Create a branch** in your fork for your changes.
3. Make your changes and commit them with clear messages.
4. **Open a Pull Request (PR)** from your branch to the `main` branch of this repository.
5. A maintainer will **review** your PR. You may be asked to make changes.
6. Once approved, your PR will be **merged** into `main`.


## Setting up your development environment

Follow the instructions in our [README](README.md#development-process) to install prerequisites and run the project locally.

## Issues & Pull Requests

When creating a new issue, use one of the available templates (Bug report, Feature request, etc.) where applicable. Make use of relevant [labels](https://github.com/UMN-Rocket-Team/WINGS/labels) in your issue and PR. 

## Commit Guidelines
- Commit frequently, they're easier to digest when you don't commit all of your changes for a new feature at the same time.
- Keep commit messages brief, yet descriptive
- Use present tense in commit messages:
    - Good example: `Add telemetry packet parser`
    - Bad example: `Added telemetry packet parser`

## Code Formatting
To ensure consistent code style, please format your code before submitting changes.
Most IDEs should have a built-in formatter for TypeScript code.

For Rust code, you can use either of the following approaches:
- **VS Code + rust-analyzer**: Install the rust-analyzer extension in VS Code. You can use this to automatically format your code on save.
- **cargo fmt**: Run `cargo fmt` command in `src-tauri` directory to format all Rust files according to standard style guidelines.

## License 
By contributing, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE.txt).