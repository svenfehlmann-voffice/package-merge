# Package Merge

`package-merge` is a Rust-based utility designed to resolve merge conflicts in `package.json` files. It parses the conflicting sections, resolves the conflicts by selecting the latest versions, and updates the `package.json` file accordingly.

## Features

- Automatically resolves version conflicts in `dependencies` and `devDependencies`.
- Ensures the resulting `package.json` is valid JSON.
- Runs `npm install` to update the lockfile after resolving conflicts.

## Installation

1. **Clone the repository**:

   ```bash
   git clone <repository-url>
   cd package-merge
   ```

2. **Build & install the project**:
   Ensure you have Rust installed. If not, install it via [rustup](https://rustup.rs/).
   ```bash
   cargo install --path .
   ```

## Usage

1. Change directory into the directory with the conflicting `package.json`
2. Run the binary:
   ```bash
   package-merge
   ```
3. The tool will:
   - Resolve conflicts in the `package.json` file.
   - Overwrite the original file with the resolved version.
   - Automatically run `npm install` to update the lockfile.

## Disclaimer

This tool automatically runs `npm install` after resolving conflicts to ensure the lockfile is updated. Please ensure you have `npm` installed and configured correctly before running the tool.
