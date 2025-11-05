/* tslint:disable */
/* eslint-disable */
export class MyCpuController {
  free(): void;
  [Symbol.dispose](): void;
  constructor();
  free(): void;
  loadProgram(assembly_string: string): boolean;
  step(): any;
  getState(): any;
  reset(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_mycpucontroller_free: (a: number, b: number) => void;
  readonly mycpucontroller_new: () => number;
  readonly mycpucontroller_free: (a: number) => void;
  readonly mycpucontroller_loadProgram: (a: number, b: number, c: number) => number;
  readonly mycpucontroller_step: (a: number) => any;
  readonly mycpucontroller_getState: (a: number) => any;
  readonly mycpucontroller_reset: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
