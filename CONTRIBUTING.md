# Contributing to WINGS

We welcome contributions from UMN rocket team members, other students, alumni, and the public.

## Development Process

Our workflow:

1. **Fork** this repository to your own GitHub account.
2. **Create a branch** in your fork for your changes.
3. Make your changes and commit them with clear messages.
4. **Open a Pull Request (PR)** from your branch to the `main` branch of this repository.
5. A maintainer will **review** your PR. You may be asked to make changes. A [GitHub Actions workflow](#github-actions-checks) will be triggered when your PR is submitted; these checks must pass for your PR to be approved.
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

## GitHub Actions Checks
When a PR is submitted, a GitHub actions workflow will be triggered. For the PR to be approved and merged, all of these
checks must pass. These checks include:

### Build Check
```
cargo build --verbose
```

### Cargo Test Run
```
cargo test --verbose
```

### Format Check
```
cargo fmt --check
```

To ensure all of the checks will pass, you should run all of these commands in your local environnment in `src-tauri` directory before 
submitting your PR. Remember, to format your code, run the `cargo fmt` command.

## License 
By contributing, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE.txt).