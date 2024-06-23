import init, { InitOutput, serve, ServerConfig, tokenize, token_type_as_js_string } from "../assets/wasm/src_lsp_browser";

import { FromServer, IntoServer } from "./codec";

let server: null | Server;

let tokenizer: null | typeof tokenize;
let as_js_string: null | typeof token_type_as_js_string;

export { tokenizer, as_js_string };

export default class Server {
  readonly initOutput: InitOutput;
  readonly #intoServer: IntoServer;
  readonly #fromServer: FromServer;

  private constructor(initOutput: InitOutput, intoServer: IntoServer, fromServer: FromServer) {
    this.initOutput = initOutput;
    this.#intoServer = intoServer;
    this.#fromServer = fromServer;
  }

  static async initialize(intoServer: IntoServer, fromServer: FromServer): Promise<Server> {
    if (null == server) {
      const initOutput = await init();
      server = new Server(initOutput, intoServer, fromServer);
      tokenizer = tokenize;
      as_js_string = token_type_as_js_string;
    } else {
      console.warn("Server already initialized; ignoring");
    }
    return server;
  }

  async start(): Promise<void> {
    const config = new ServerConfig(this.#intoServer, this.#fromServer);
    await serve(config);
  }
}
