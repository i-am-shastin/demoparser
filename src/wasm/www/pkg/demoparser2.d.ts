/* tslint:disable */
/* eslint-disable */
/**
 * @param {Uint8Array} file
 * @param {string} event_name
 * @param {(string)[] | undefined} [player_props]
 * @param {(string)[] | undefined} [other_props]
 * @returns {any}
 */
export function parseEvent(file: Uint8Array, event_name: string, player_props?: (string)[], other_props?: (string)[]): any;
/**
 * @param {Uint8Array} file
 * @param {(string)[]} event_names
 * @param {(string)[] | undefined} [player_props]
 * @param {(string)[] | undefined} [other_props]
 * @returns {any}
 */
export function parseEvents(file: Uint8Array, event_names: (string)[], player_props?: (string)[], other_props?: (string)[]): any;
/**
 * @param {Uint8Array} file
 * @param {(string)[]} wanted_props
 * @param {Int32Array | undefined} [wanted_ticks]
 * @param {(string)[] | undefined} [wanted_players]
 * @param {boolean | undefined} [struct_of_arrays]
 * @param {boolean | undefined} [order_by_steamid]
 * @param {(WantedPropState)[] | undefined} [prop_states]
 * @returns {any}
 */
export function parseTicks(file: Uint8Array, wanted_props: (string)[], wanted_ticks?: Int32Array, wanted_players?: (string)[], struct_of_arrays?: boolean, order_by_steamid?: boolean, prop_states?: (WantedPropState)[]): any;
/**
 * @param {Uint8Array} file
 * @returns {any}
 */
export function listGameEvents(file: Uint8Array): any;
/**
 * @param {Uint8Array} file
 * @returns {any}
 */
export function parseGrenades(file: Uint8Array): any;
/**
 * @param {Uint8Array} file
 * @returns {any}
 */
export function parseHeader(file: Uint8Array): any;
export class WantedPropState {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wantedpropstate_free: (a: number, b: number) => void;
  readonly parseEvent: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly parseEvents: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly parseTicks: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => void;
  readonly listGameEvents: (a: number, b: number, c: number) => void;
  readonly parseGrenades: (a: number, b: number, c: number) => void;
  readonly parseHeader: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
