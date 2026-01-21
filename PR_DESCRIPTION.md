# Add static and shared library support

Adds support for building `tree_magic_mini` as static and shared libraries with C FFI bindings.

- Added C FFI wrappers for all public APIs
- Updated `Cargo.toml` to build `cdylib` and `staticlib` crate types
- Added Nix package with C header generation via `cbindgen` and pkg-config support
