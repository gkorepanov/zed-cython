# zed-cython

[Cython](https://cython.org/) language support for [Zed](https://zed.dev/) based on [tree-sitter-cython](https://github.com/b0o/tree-sitter-cython).

Includes syntax highlighting, outline, indentation, and optional LSP support via [Cyright](https://github.com/ktnrg45/cyright).

## Prebuilt ZIP

For a no-Rust/no-npm install, use a prebuilt ZIP:

```sh
unzip zed-cython-cyright-*.zip
cd zed-cython-cyright-*
./install_macos.sh
```

Restart Zed and open a `.pyx`, `.pxd`, or `.pxi` file. The bundle includes Cyright and runs it through Zed's Node runtime.

## Cyright LSP

Build Cyright from the VS Code extension repository:

```sh
git clone https://github.com/ktnrg45/vs-code-cython.git
cd vs-code-cython
git submodule update --init --recursive cyright
cd cyright
npm install
npm run build:cli:dev
```

Then configure Zed:

```jsonc
{
  "lsp": {
    "cyright": {
      "binary": {
        "path": "/absolute/path/to/vs-code-cython/cyright/packages/pyright/langserver.index.js",
        "arguments": ["--stdio"]
      },
      "settings": {
        "python": {
          "pythonPath": "/absolute/path/to/python"
        },
        "python.analysis": {
          "extraPaths": []
        },
        "cython": {
          "includePaths": []
        }
      }
    }
  }
}
```

If the server does not start automatically, enable it for Cython explicitly:

```jsonc
{
  "languages": {
    "Cython": {
      "language_servers": ["cyright"]
    }
  }
}
```

## Build ZIP

For maintainers:

```sh
CYRIGHT_DIR=/absolute/path/to/vs-code-cython/cyright ./scripts/build_distributable.sh
```

The script expects Cyright to be built with `npm run build:cli:dev`.

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
