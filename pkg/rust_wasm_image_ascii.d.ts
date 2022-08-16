/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} raw
* @param {number} scale
* @returns {Uint8Array}
*/
export function get_gray_image(raw: Uint8Array, scale: number): Uint8Array;
/**
* @param {Uint8Array} raw
* @param {number} scale
* @param {boolean} revert
* @returns {string}
*/
export function get_ascii_by_image(raw: Uint8Array, scale: number, revert: boolean): string;
/**
*/
export function run(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_gray_image: (a: number, b: number, c: number, d: number) => void;
  readonly get_ascii_by_image: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly run: () => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_start: () => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
