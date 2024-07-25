# Getting Started

## Build Shared Library

> [!WARNING]
>
> The shared library is places inside of `src/main/resources/` and is **not** committed to the git repository (see `.gitignore`), and so therefore in order to develop with this project, you must generate the binding using the following command.
>
> Currently, the `Justfile` assumes the developer machine is *macOS running on Apple Silicon*, though additional support is forthcoming.

```shell
(cd ../../ && just bind)
```

