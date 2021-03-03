// @ts-ignore
let wasmInstance = import("../../wasm_export/pkg")
export type WasmInstance = typeof import("../../wasm_export/pkg/index");

export const getWasmInstance = () => wasmInstance as Promise<WasmInstance>
