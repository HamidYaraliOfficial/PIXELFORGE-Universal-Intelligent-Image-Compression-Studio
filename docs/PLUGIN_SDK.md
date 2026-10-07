# PIXELFORGE Plugin SDK

A codec plugin is an implementation of a versioned adapter boundary:

```text
PluginMetadata
  id
  name
  version
  capabilities
  supported_extensions
  encode(request) -> result
  decode(request) -> result
```

The production extension path may be implemented as:
- Rust dynamic libraries (`cdylib`) exposing a C ABI.
- Sandboxed helper processes communicating over JSON-RPC.
- Native codec wrappers linked through CMake/vcpkg on Windows and pkg-config on Linux.

Plugins must validate untrusted dimensions, refuse integer overflow, cap allocations, and report deterministic errors.
