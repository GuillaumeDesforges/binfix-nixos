# binfix-nixos

Make arbitrary binaries work on NixOS.

> This is a WIP.

Includes a special mode for Python venv.

## Usage


Get binfix using Nix.

```console
$ nix shell GuillaumeDesforges#binfix-nixos
```

Patch some NodeJS executable.

```console
$ binfix --libs zlib ./node
```

Patch with multiple libraries.

```console
$ binfix --libs zlib,cffi ./node
```

Patch a Python virtual environment.

```console
$ binfix --libs zlib --python .venv/
```

Include GPU drivers.

```console
$ binfix --libs zlib --python --gpu .venv/
```

Use a specific version of Nixpkgs.

```console
$ binfix --nixpkgs github:nixos/nixpkgs --stdenv --libs zlib --python .venv/
```

## TODOs

- [ ] patch a binary file
- [ ] include Python venv distributed binaries
- [ ] add GPU flag
- [ ] set nixpkgs version
- [ ] automatically search for the right packages
