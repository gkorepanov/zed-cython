# zed-cython

[Cython](https://cython.org/) language support for [Zed](https://zed.dev/) based on [tree-sitter-cython](https://github.com/b0o/tree-sitter-cython).

Includes syntax highlighting, outline, indentation, and optional LSP support via [Cyright](https://github.com/ktnrg45/cyright).

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

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
