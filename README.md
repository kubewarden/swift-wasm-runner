> **Project temporarily archived:** currently swiftwasm suffers from stability issues
> We are going to archive this project until these issues are solved

# swift-wasm-runner

This is a simple Wasm runner built on top of Wasmtime. It behaves exactly like
the `wasmtime` and the `wasmer` cli tools.

It has been built to allow unit testing of Kubewarden policies written using
the Swift programming language.

## Why?

The Kubewarden policies written using Swift leverage the [`policy-sdk-swift`](https://github.com/kubewarden/policy-sdk-swift)
library, which depends on the [`wapc-guest-swift`](https://github.com/wapc/wapc-guest-swift) library.

The Wasm modules built will then import a series of [waPC](https://wapc.io/)
related functions. These functions must be provided by the Wasm host when
the unit tests are run. This causes tools like `wasmer` and `wasmtime`
to fail at runtime due to the unresolved imports.

This project provides a a simple Wasm host that can satisfy these function import
requirements.

## How it works

This project provides a simple Wasm host built on top of Wasmtime that load
and evaluate a Wasm module.

The WASI snapshot 1 and the waPC functions are made available to the Wasm module.
The waPC ones are simple mocks that do not perform anything real. This is good
enough for running the unit tests of a Kubewarden policy.

The Wasm module has access only to the STDOUT and STDERR of the host.

## How to use it

Kubewarden Swift policies provide a Makefile target that runs the unit
tests inside of a container image.

This repository provides a container image that is based on the
`ghcr.io/swiftwasm/swiftwasm-action:5.3` one, but includes the
`swift-policy-runner` instead of the default `wasmer` binary.

Nothing has to be done if you created your Kubewarden Swift policy starting
from [our template](https://github.com/kubewarden/swift-policy-template).
