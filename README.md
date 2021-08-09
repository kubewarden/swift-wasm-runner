# swift-wasm-runner

This is a simple Wasm runner built on top of Wasmtime. It behaves exactly like
the `wasmtime` and the `wasmer` cli tools.

It has been built to allow unit testing of Kubewarden policies writting using
the Swift programming language.

## Why?

The Kubewarden policies writting using Swift leverage the [`policy-sdk-swift`](https://github.com/kubewarden/policy-sdk-swift)
library, which depends on the [`wapc-guest-swift`](https://github.com/wapc/wapc-guest-swift) library.

The Wasm modules built will then import a series of [waPC](https://wapc.io/)
related functions. All the waPC related functions are going to be imported
by the module, even if they are not used.

Even the unit tests of the Swift code will require these functions, which poses
a problem because neither `wasmer` nor `wasmtime` provide them.

This is the reason that lead to the creation of this simple Wasm host.

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
from our template.
