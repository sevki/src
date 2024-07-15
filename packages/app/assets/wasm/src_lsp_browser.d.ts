/* tslint:disable */
/* eslint-disable */
/**
* @param {ServerConfig} config
* @returns {Promise<void>}
*/
export function serve(config: ServerConfig): Promise<void>;
/**
* @param {TokenType} token_type
* @returns {string}
*/
export function token_type_as_js_string(token_type: TokenType): string;
/**
* @param {string} input
* @returns {(TokenSpan)[]}
*/
export function tokenize(input: string): (TokenSpan)[];
/**
*/
export enum TokenType {
  Pipe = 0,
  Ampersand = 1,
  Semicolon = 2,
  Equals = 3,
  LessThan = 4,
  GreaterThan = 5,
  Variable = 6,
  Word = 7,
  String = 8,
  Comment = 9,
  Integer = 10,
  Float = 11,
  Eof = 12,
  NewLine = 13,
  LeftParen = 14,
  RightParen = 15,
  LeftBrace = 16,
  RightBrace = 17,
  LeftBracket = 18,
  RightBracket = 19,
  Comma = 20,
  Dot = 21,
  Colon = 22,
  Underscore = 23,
  Minus = 24,
  Plus = 25,
  Arrow = 26,
  FatArrow = 27,
  Divide = 28,
  Multiply = 29,
  Percent = 30,
  Dollar = 31,
  Exclamation = 32,
  Question = 33,
  Tilde = 34,
  At = 35,
  Caret = 36,
  Shebang = 37,
}
/**
*/
export class IntoUnderlyingByteSource {
  free(): void;
/**
* @param {any} controller
*/
  start(controller: any): void;
/**
* @param {any} controller
* @returns {Promise<any>}
*/
  pull(controller: any): Promise<any>;
/**
*/
  cancel(): void;
/**
*/
  readonly autoAllocateChunkSize: number;
/**
*/
  readonly type: any;
}
/**
*/
export class IntoUnderlyingSink {
  free(): void;
/**
* @param {any} chunk
* @returns {Promise<any>}
*/
  write(chunk: any): Promise<any>;
/**
* @returns {Promise<any>}
*/
  close(): Promise<any>;
/**
* @param {any} reason
* @returns {Promise<any>}
*/
  abort(reason: any): Promise<any>;
}
/**
*/
export class IntoUnderlyingSource {
  free(): void;
/**
* @param {any} controller
* @returns {Promise<any>}
*/
  pull(controller: any): Promise<any>;
/**
*/
  cancel(): void;
}
/**
* Raw options for [`pipeTo()`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo).
*/
export class PipeOptions {
  free(): void;
/**
*/
  readonly preventAbort: boolean;
/**
*/
  readonly preventCancel: boolean;
/**
*/
  readonly preventClose: boolean;
/**
*/
  readonly signal: AbortSignal | undefined;
}
/**
*/
export class QueuingStrategy {
  free(): void;
/**
*/
  readonly highWaterMark: number;
}
/**
* Raw options for [`getReader()`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader).
*/
export class ReadableStreamGetReaderOptions {
  free(): void;
/**
*/
  readonly mode: any;
}
/**
*/
export class ServerConfig {
  free(): void;
/**
* @param {AsyncIterator<any>} into_server
* @param {WritableStream} from_server
*/
  constructor(into_server: AsyncIterator<any>, from_server: WritableStream);
}
/**
*/
export class TokenSpan {
  free(): void;
/**
*/
  end: number;
/**
*/
  scope: TokenType;
/**
*/
  start: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_serverconfig_free: (a: number) => void;
  readonly serverconfig_new: (a: number, b: number) => number;
  readonly serve: (a: number) => number;
  readonly __wbg_tokenspan_free: (a: number) => void;
  readonly __wbg_get_tokenspan_start: (a: number) => number;
  readonly __wbg_set_tokenspan_start: (a: number, b: number) => void;
  readonly __wbg_get_tokenspan_end: (a: number) => number;
  readonly __wbg_set_tokenspan_end: (a: number, b: number) => void;
  readonly __wbg_get_tokenspan_scope: (a: number) => number;
  readonly __wbg_set_tokenspan_scope: (a: number, b: number) => void;
  readonly token_type_as_js_string: (a: number) => number;
  readonly tokenize: (a: number, b: number, c: number) => void;
  readonly __wbg_intounderlyingsink_free: (a: number) => void;
  readonly intounderlyingsink_write: (a: number, b: number) => number;
  readonly intounderlyingsink_close: (a: number) => number;
  readonly intounderlyingsink_abort: (a: number, b: number) => number;
  readonly __wbg_intounderlyingsource_free: (a: number) => void;
  readonly intounderlyingsource_pull: (a: number, b: number) => number;
  readonly intounderlyingsource_cancel: (a: number) => void;
  readonly __wbg_readablestreamgetreaderoptions_free: (a: number) => void;
  readonly readablestreamgetreaderoptions_mode: (a: number) => number;
  readonly __wbg_pipeoptions_free: (a: number) => void;
  readonly pipeoptions_preventClose: (a: number) => number;
  readonly pipeoptions_preventCancel: (a: number) => number;
  readonly pipeoptions_preventAbort: (a: number) => number;
  readonly pipeoptions_signal: (a: number) => number;
  readonly __wbg_queuingstrategy_free: (a: number) => void;
  readonly queuingstrategy_highWaterMark: (a: number) => number;
  readonly __wbg_intounderlyingbytesource_free: (a: number) => void;
  readonly intounderlyingbytesource_type: (a: number) => number;
  readonly intounderlyingbytesource_autoAllocateChunkSize: (a: number) => number;
  readonly intounderlyingbytesource_start: (a: number, b: number) => void;
  readonly intounderlyingbytesource_pull: (a: number, b: number) => number;
  readonly intounderlyingbytesource_cancel: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hef4ca354c23a4fc7: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__hd246805203d2bb11: (a: number, b: number, c: number, d: number) => void;
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
