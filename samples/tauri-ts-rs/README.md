# Tauri + ts-rs Sample

This sample mirrors the common Tauri app setup:

- Rust models derive `ts_rs::TS`;
- `typeship-ts-rs` lowers those models into declarations;
- `typeship` assembles those declarations with typed `invoke` command wrappers;
- the tiny CLI driver writes or drift-checks the generated TypeScript file.

From the repository root:

```sh
cargo run -p typeship-sample-tauri-ts-rs -- write
cargo run -p typeship-sample-tauri-ts-rs -- check
```

The generated file is committed at `samples/tauri-ts-rs/generated/api.ts`.
