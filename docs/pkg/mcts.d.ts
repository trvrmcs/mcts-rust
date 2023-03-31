/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
*/
export class Connect4Game {
  free(): void;
/**
*/
  constructor();
/**
*/
  reset(): void;
/**
* @param {number} i
* @param {number} j
* @returns {string}
*/
  cell(i: number, j: number): string;
/**
* @returns {string}
*/
  player(): string;
/**
* @returns {string}
*/
  result(): string;
/**
* @param {number} n
* @returns {number}
*/
  suggest_move(n: number): number;
/**
* @param {number} column
* @returns {string}
*/
  apply(column: number): string;
}
/**
*/
export class TicTacToeCommand {
  free(): void;
/**
*/
  i: number;
/**
*/
  j: number;
}
/**
*/
export class TicTacToeGame {
  free(): void;
/**
*/
  constructor();
/**
*/
  reset(): void;
/**
* @returns {number}
*/
  size(): number;
/**
* @param {number} i
* @param {number} j
* @returns {string}
*/
  cell(i: number, j: number): string;
/**
* @returns {string}
*/
  player(): string;
/**
* @returns {string}
*/
  result(): string;
/**
* @param {number} n
* @returns {TicTacToeCommand}
*/
  suggest_move(n: number): TicTacToeCommand;
/**
* @param {number} column
* @param {number} row
* @returns {string}
*/
  apply(column: number, row: number): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: (a: number, b: number) => void;
  readonly __wbg_connect4game_free: (a: number) => void;
  readonly connect4game_new: () => number;
  readonly connect4game_reset: (a: number) => void;
  readonly connect4game_cell: (a: number, b: number, c: number, d: number) => void;
  readonly connect4game_player: (a: number, b: number) => void;
  readonly connect4game_result: (a: number, b: number) => void;
  readonly connect4game_suggest_move: (a: number, b: number) => number;
  readonly connect4game_apply: (a: number, b: number, c: number) => void;
  readonly __wbg_tictactoegame_free: (a: number) => void;
  readonly __wbg_tictactoecommand_free: (a: number) => void;
  readonly __wbg_get_tictactoecommand_i: (a: number) => number;
  readonly __wbg_set_tictactoecommand_i: (a: number, b: number) => void;
  readonly __wbg_get_tictactoecommand_j: (a: number) => number;
  readonly __wbg_set_tictactoecommand_j: (a: number, b: number) => void;
  readonly tictactoegame_new: () => number;
  readonly tictactoegame_reset: (a: number) => void;
  readonly tictactoegame_size: (a: number) => number;
  readonly tictactoegame_cell: (a: number, b: number, c: number, d: number) => void;
  readonly tictactoegame_player: (a: number, b: number) => void;
  readonly tictactoegame_result: (a: number, b: number) => void;
  readonly tictactoegame_suggest_move: (a: number, b: number) => number;
  readonly tictactoegame_apply: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
