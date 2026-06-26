# Basic IR Sample

This sample uses only the zero-dependency `typeship` core crate. It hand-builds
a tiny task API with `Decl`, `Field`, `TsType`, and `Command`, then renders a
transport-agnostic client surface that expects the TypeScript consumer to provide
a `request(command, payload)` helper.

From the repository root:

```sh
cargo run -p typeship-sample-basic-ir -- write
cargo run -p typeship-sample-basic-ir -- check
```

The generated file is committed at `samples/basic-ir/generated/api.ts` so the
`check` command can be used as a CI drift guard.
