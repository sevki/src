// import * as jsrpc from "json-rpc-2.0";
import {
  MonacoToProtocolConverter,
  ProtocolToMonacoConverter,
} from "monaco-languageclient";
import * as monaco from "monaco-editor-core";
import * as proto from "vscode-languageserver-protocol";

import Client from "./client";
import { tokenizer, as_js_string } from "./server";

export const monacoToProtocol = new MonacoToProtocolConverter(monaco);
export const protocolToMonaco = new ProtocolToMonacoConverter(monaco);

let language: null | Language;

export default class Language
  implements monaco.languages.ILanguageExtensionPoint
{
  readonly id: string;
  readonly aliases: string[];
  readonly extensions: string[];
  readonly mimetypes: string[];

  private constructor(client: Client) {
    const { id, aliases, extensions, mimetypes } = Language.extensionPoint();
    this.id = id;
    this.aliases = aliases;
    this.extensions = extensions;
    this.mimetypes = mimetypes;
    this.registerLanguage(client);
  }

  static extensionPoint(): monaco.languages.ILanguageExtensionPoint & {
    aliases: string[];
    extensions: string[];
    mimetypes: string[];
  } {
    const id = "src lang";
    const aliases = ["src"];
    const extensions = [".src"];
    const mimetypes = ["text/src"];
    return { id, extensions, aliases, mimetypes };
  }

  private registerLanguage(client: Client): void {
    void client;
    monaco.languages.register(Language.extensionPoint());
    monaco.languages.registerDocumentSymbolProvider(this.id, {
      // eslint-disable-next-line
      async provideDocumentSymbols(
        model,
        token
      ): Promise<monaco.languages.DocumentSymbol[]> {
        void token;
        const response = await (client.request(
          proto.DocumentSymbolRequest.type.method,
          {
            textDocument: monacoToProtocol.asTextDocumentIdentifier(model),
          } as proto.DocumentSymbolParams
        ) as Promise<proto.SymbolInformation[]>);

        const uri = model.uri.toString();

        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const result: monaco.languages.DocumentSymbol[] =
          protocolToMonaco.asSymbolInformations(response, uri);

        return result;
      },
    });
    monaco.languages.registerDocumentHighlightProvider(this.id, {
      // eslint-disable-next-line
      async provideDocumentHighlights(
        model,
        position,
        token
      ): Promise<monaco.languages.DocumentHighlight[]> {
        void token;
        const response = await (client.request(
          proto.DocumentHighlightRequest.type.method,
          {
            textDocument: monacoToProtocol.asTextDocumentIdentifier(model),
            position: monacoToProtocol.asPosition(
              position.lineNumber,
              position.column
            ),
          } as proto.DocumentHighlightParams
        ) as Promise<proto.DocumentHighlight[]>);

        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const result: monaco.languages.DocumentHighlight[] =
          protocolToMonaco.asDocumentHighlights(response);

        return result;
      },
    });
    monaco.languages.registerHoverProvider(this.id, {
      // eslint-disable-next-line
      async provideHover(
        model,
        position,
        token
      ): Promise<monaco.languages.Hover> {
        void token;
        const response = await (client.request(proto.HoverRequest.type.method, {
          textDocument: monacoToProtocol.asTextDocumentIdentifier(model),
          position: monacoToProtocol.asPosition(
            position.lineNumber,
            position.column
          ),
        } as proto.HoverParams) as Promise<proto.Hover>);

        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const result: monaco.languages.Hover =
          protocolToMonaco.asHover(response);

        return result;
      },
    });
    monaco.languages.registerDocumentSemanticTokensProvider(this.id, {
      // eslint-disable-next-line
      async provideDocumentSemanticTokens(
        model: monaco.editor.ITextModel,
        lastResultId: string | null,
        token: monaco.CancellationToken
      ): Promise<
        | monaco.languages.SemanticTokens
        | monaco.languages.SemanticTokensEdits
        | null
        | undefined
      > {
        void lastResultId;
        void token;
        const response = await (client.request(
          proto.SemanticTokensRequest.type.method,
          {
            textDocument: monacoToProtocol.asTextDocumentIdentifier(model),
          } as proto.SemanticTokensParams
        ) as Promise<proto.SemanticTokens>);

        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const result: monaco.languages.SemanticTokens =
          protocolToMonaco.asSemanticTokens(response);

        return result;
      },
      getLegend: function (): monaco.languages.SemanticTokensLegend {
        console.log("getLegend");
        return {
          tokenTypes: [],
          tokenModifiers: [],
        };
      },
      releaseDocumentSemanticTokens: function (
        resultId: string | undefined
      ): void {
        console.log("releaseDocumentSemanticTokens");
        void resultId;
      },
    });

    monaco.languages.setTokensProvider(this.id, {
      // eslint-disable-next-line
      getInitialState(): monaco.languages.IState {
        return {
          clone: function (): monaco.languages.IState {
            return this;
          },
          equals: function (other: monaco.languages.IState): boolean {
            return true;
          },
        };
      },
      // eslint-disable-next-line
      tokenize(
        line: string,
        state: monaco.languages.IState
      ): monaco.languages.ILineTokens {
        if (tokenizer) {
          const result = tokenizer(line);
          let tokens = result.map((token) => {
            let scope = as_js_string ? as_js_string(token.scope) : "";
            return {
              startIndex: token.start,
              scopes: scope,
            };
          });
          console.log(tokens);
          return {
            endState: state,
            tokens,
          };
        }
        return {
          endState: state,
          tokens: [],
        };
      },
    });
  }

  static initialize(client: Client): Language {
    if (null == language) {
      language = new Language(client);
    } else {
      console.warn("Language already initialized; ignoring");
    }
    return language;
  }
}
