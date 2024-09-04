
# Rust CLI Utilities

Welcome to the Rust CLI Utilities project! This repository contains a collection of command-line utilities written in Rust, inspired by GNU CLI tools such as `echo`, `ls`, and more. It also includes custom utilities developed specifically for this project.

## Features

- **Rust-based CLI Tools:** Utilities similar to standard GNU commands, implemented in Rust for performance and reliability.
- **Custom Utilities:** Unique tools created to solve specific problems or improve workflows.

## Existing Utilities

- **r-todolist**(dev-v0.1) :r-todolist is a command-line tool for managing tasks and to-do lists, powered by SQLite for serverless database persistence.
- **r-echo**(dev): echo in rust .
- **r-vim**(dev): vim minimal clone in rust .

## Getting Started

To get started with the Rust CLI Utilities, follow these steps:

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (including `cargo`)

### Building the Project

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/your-username/rust-cli-utilities.git
   cd rust-cli-utilities
   ```

2. **Build the Project:**

   ```bash
   cargo build --release
   ```

   This will compile the CLI tools and place the binaries in the `target/release` directory.

### Adding the Executable to `PATH`

To make the CLI tools accessible from anywhere, add the path to the `target/release` directory to your `PATH` environment variable.

#### Temporary Addition (Current Terminal Session Only)

```bash
export PATH=$PATH:/path/to/rust-cli-utilities/target/release
```

Replace `/path/to/rust-cli-utilities` with the path to your project directory.

#### Permanent Addition (All Terminal Sessions)

1. **Identify Your Shell Configuration File:**

   - **Bash:** `~/.bashrc` or `~/.bash_profile`
   - **Zsh:** `~/.zshrc`
   - **Fish:** `~/.config/fish/config.fish`

2. **Edit the Configuration File:**

   Open the appropriate file with a text editor. For example, for Bash:

   ```bash
   nano ~/.bashrc
   ```

   For Zsh:

   ```bash
   nano ~/.zshrc
   ```

3. **Add the Path to `PATH`:**

   Append the following line to the end of the file:

   ```bash
   export PATH=$PATH:/path/to/rust-cli-utilities/target/release
   ```

4. **Apply the Changes:**

   After saving the file, apply the changes by sourcing the file. For example, for Bash:

   ```bash
   source ~/.bashrc
   ```

   For Zsh:

   ```bash
   source ~/.zshrc
   ```

## Usage

After building the project and adding the binaries to your `PATH`, you can use the CLI tools from any terminal session.

### Example Commands

- **`echo`:** A simple command to print text to the console.
- **`ls`:** Lists files and directories.

For a list of available commands and their usage, refer to the specific documentation for each tool.

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests for any changes or improvements.

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes.
4. Push to the feature branch.
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

If you have any questions or feedback, feel free to open an issue on the GitHub repository or contact the project maintainer.

Happy coding!

