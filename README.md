# ito-pkg

A CLI tool for developing, packaging, and hosting plugins for the Ito application.

## Installation

```bash
cargo install --path .
```

## Usage

### 1. Create a New Plugin

Scaffold a new plugin project. Supported types are `manga`, `anime`, and `novel`.

```bash
ito-pkg new <plugin-name> --type <type>
# Or use the short flag
ito-pkg new <plugin-name> -t <type>
```

Example:
```bash
ito-pkg new my-manga-source -t manga
```

This creates a new directory with:
- `Cargo.toml`: Configured for `cdylib` compilation.
- `manifest.json`: Plugin metadata.
- `src/lib.rs`: Starter code implementing the provider trait.

### 2. Develop Your Plugin

Navigate to your plugin directory and implement the required trait methods in `src/lib.rs`.

Add an icon to your plugin by placing an `icon.png` file in the project root.

### 3. Package the Plugin

Compile your plugin to WebAssembly and bundle it into an `.ito` file.

```bash
ito-pkg pack
```

Output:
- `target/wasm32-unknown-unknown/release/<name>.wasm`: The compiled binary.
- `<name>.ito`: The final distributable package containing the Wasm binary, manifest, and icon.

### 4. Verify a Package

Check the contents and validity of an `.ito` file.

```bash
ito-pkg verify <path-to-ito-file>
```

### 5. Create a Local Repository

Organize multiple `.ito` files into a static repository structure that the Ito app can consume.

```bash
# Create a folder for your built plugins
mkdir plugins
mv *.ito plugins/

# Build the repository
ito-pkg repo --input plugins --output public --url "http://localhost:8080"
```

This generates:
- `public/index.json`: Repository index.
- `public/index.min.json`: Minified index.
- `public/packages/`: Directory containing `.ito` files.
- `public/icons/`: Directory containing extracted icons.

### 6. Serve the Repository

Host your repository locally for testing.

```bash
ito-pkg serve --path public --port 8080
```

Add `http://localhost:8080/index.json` to the Ito app to install your plugins.
