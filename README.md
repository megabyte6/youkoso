# Youkoso

Welcome to **Youkoso**, a modern and customizable application designed to streamline your workflow with a sleek user interface powered by [Slint](https://slint.dev/) and a robust backend written in Rust. This project combines performance, flexibility, and simplicity to deliver a seamless user experience.

---

## Features

- **Dynamic Theme Support**: Switch between `Light`, `Dark`, and `System` themes effortlessly.
- **Configuration Management**: Load and save settings using a TOML-based configuration file.
- **API Integration**: Interact with external APIs securely using `reqwest`.
- **Customizable UI**: Built with Slint for a responsive and visually appealing interface.
- **Error Handling**: Comprehensive error management for robust and reliable performance.

---

## Getting Started

### Download
You can download the pre-built executables from the [Releases](https://github.com/megabyte6/youkoso/releases) page on GitHub. 

1. Navigate to the [Releases](https://github.com/megabyte6/youkoso/releases) page.
2. Locate the latest release and download the appropriate executable for your operating system.
3. Once downloaded, extract the contents (if necessary) and run the executable.

Enjoy using **Youkoso**!

---

## Build It Yourself

### Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

---

### Build

1. Clone the repository:
   ```bash
   git clone https://github.com/megabyte6/youkoso.git
   ```
1. Open the project:
   ```bash
   cd youkoso
   ```
1. Build with cargo:
   ```bash
   cargo build --release
   ```

The binary will be found in `target/release/`

---

### Using Different Slint Styles

Slint supports multiple rendering styles, such as `fluent` and `material`. Slint will, by default, choose which style to use based on your OS but you can specify the desired style by setting the `SLINT_STYLE` environment variable before building the project.

**Styles**:
- `fluent`
- `material`
- `cupertino`
- `cosmic`
- `qt` (requires Qt to be installed on your system)

**Example** (for Linux and macOS):

To build the application with the `material` style:
```bash
SLINT_STYLE=material cargo build --release
```

To use the `fluent` style:
```bash
SLINT_STYLE=fluent cargo build --release
```

---

## Configuration
The application has a settings page but the `config.toml` can be used for manual settings management. Below is an example configuration:

```toml
theme = "System" # Options: "System", "Dark", "Light"

[my_studio]
email = "user@example.com"
password = "your_password"
company_id = "12345"
```

Place the config.toml file in the same directory as the executable.

---

## Issues

If you encounter any issues while using **Youkoso**, feel free to open an issue on the [Issues](https://github.com/megabyte6/youkoso/issues) page. Provide as much detail as possible, including steps to reproduce the issue, your operating system, and any relevant logs or screenshots.

---

## Contributing
Contributions are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository.
1. Clone your fork.
1. Create a new branch:
   ```bash
   git branch feature-name
   ```
1. Make your changes and commit them:
   ```bash
   git commit -am "Add feature-name"
   ```
1. Push to your branch:
   ```bash
   git push origin feature-name
   ```
1. Open a pull request.

---

## License
This project is licensed under the [Apache License 2.0](https://github.com/megabyte6/youkoso/blob/main/LICENSE).

---

## Acknowledgments
- Slint for the UI framework.
- Rust for the powerful backend.
