/* tslint:disable */
/* eslint-disable */

export class BenchmarkResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    avg: number;
    max: number;
    min: number;
    total: number;
    tps: number;
}

export enum Cell {
    Alive = 1,
    Dead = 0,
}

export class Universe {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cells(): number;
    height(): number;
    static new(width: number, height: number): Universe;
    pixel_buffer_ptr(): number;
    render(): void;
    tick(): void;
    width(): number;
}

export function main(): void;

export function run_benchmark(size: number, iterations: number): BenchmarkResult;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_benchmarkresult_free: (a: number, b: number) => void;
    readonly __wbg_get_benchmarkresult_avg: (a: number) => number;
    readonly __wbg_get_benchmarkresult_max: (a: number) => number;
    readonly __wbg_get_benchmarkresult_min: (a: number) => number;
    readonly __wbg_get_benchmarkresult_total: (a: number) => number;
    readonly __wbg_get_benchmarkresult_tps: (a: number) => number;
    readonly __wbg_set_benchmarkresult_avg: (a: number, b: number) => void;
    readonly __wbg_set_benchmarkresult_max: (a: number, b: number) => void;
    readonly __wbg_set_benchmarkresult_min: (a: number, b: number) => void;
    readonly __wbg_set_benchmarkresult_total: (a: number, b: number) => void;
    readonly __wbg_set_benchmarkresult_tps: (a: number, b: number) => void;
    readonly __wbg_universe_free: (a: number, b: number) => void;
    readonly main: () => void;
    readonly run_benchmark: (a: number, b: number) => number;
    readonly universe_cells: (a: number) => number;
    readonly universe_height: (a: number) => number;
    readonly universe_new: (a: number, b: number) => number;
    readonly universe_pixel_buffer_ptr: (a: number) => number;
    readonly universe_render: (a: number) => void;
    readonly universe_tick: (a: number) => void;
    readonly universe_width: (a: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
