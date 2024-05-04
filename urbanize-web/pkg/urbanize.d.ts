/* tslint:disable */
/* eslint-disable */
/**
*/
export function main(): void;
/**
* @param {string} s
* @param {string} id
*/
export function set_points(s: string, id: string): void;
/**
* @param {string} id
*/
export function remove_road(id: string): void;
/**
* @param {string} id
*/
export function remove_zone(id: string): void;
/**
* @param {string} id
*/
export function add_road(id: string): void;
/**
* @param {string} id
*/
export function add_zone(id: string): void;
/**
*/
export function update(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly set_points: (a: number, b: number, c: number, d: number) => void;
  readonly remove_road: (a: number, b: number, c: number) => void;
  readonly remove_zone: (a: number, b: number, c: number) => void;
  readonly add_road: (a: number, b: number, c: number) => void;
  readonly add_zone: (a: number, b: number, c: number) => void;
  readonly update: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
