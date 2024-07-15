import debounce from "debounce";
import * as monaco from "monaco-editor-core";
import { MonacoToProtocolConverter } from "monaco-languageclient";
import * as proto from "vscode-languageserver-protocol";

import Client from "./client";
import { FromServer, IntoServer } from "./codec";
import Language from "./language";
import Server from "./server";

class Environment implements monaco.Environment {
  getWorkerUrl(moduleId: string, label: string) {
    if (label === "editorWorkerService") {
      return "./editor.worker.bundle.js";
    }
    throw new Error(
      `getWorkerUrl: unexpected ${JSON.stringify({ moduleId, label })}`
    );
  }
}

const monacoToProtocol = new MonacoToProtocolConverter(monaco);
function commonOptions(theme: "light" | "dark") {
  const isLight = theme === "light";

  // Vibrant Color Palette (tailored for light/dark)
  const vibrantColors = {
    red: isLight ? "#FF2400" : "#FF5C33",     
    green: isLight ? "#1AFF00" : "#33FF33",    
    blue: isLight ? "#0040FF" : "#3399FF",    
    yellow: isLight ? "#FFFF00" : "#FFFF66",   
    purple: isLight ? "#9933FF" : "#CC99FF",    
    orange: isLight ? "#FF9900" : "#FFCC66",    
  };

  return {
    rules: [
      { token: "Pipe", foreground: vibrantColors.red },
      { token: "Ampersand", foreground: vibrantColors.green },
      { token: "Semicolon", foreground: vibrantColors.blue },
      { token: "Equals", foreground: vibrantColors.yellow },
      { token: "LessThan", foreground: vibrantColors.purple },
      { token: "GreaterThan", foreground: vibrantColors.orange },
      { token: "Variable", foreground: vibrantColors.orange },
      { token: "Word", foreground: isLight ? "#000000" : "#FFFFFF" },
      { token: "String", foreground: vibrantColors.red, fontStyle: "italic" },
      { token: "Comment", foreground: vibrantColors.green, fontStyle: "italic" },
      { token: "Integer", foreground: vibrantColors.blue },
      { token: "Float", foreground: vibrantColors.yellow },
      { token: "Eof", foreground: vibrantColors.purple },
      { token: "NewLine", foreground: vibrantColors.orange },

      // Bracket Color Pairs (with darker right counterparts)
      { token: "LeftParen", foreground: vibrantColors.red },
      { token: "RightParen", foreground: "#8B0000" }, // Maroon 
      { token: "LeftBrace", foreground: vibrantColors.green },
      { token: "RightBrace", foreground: "#006400" }, // Dark green
      { token: "LeftBracket", foreground: vibrantColors.blue },
      { token: "RightBracket", foreground: "#00008B" }, // Dark blue
      { token: "Comma", foreground: vibrantColors.purple },
      { token: "Dot", foreground: vibrantColors.orange },
      { token: "Colon", foreground: vibrantColors.orange },
      { token: "Underscore", foreground: "#888888" },
      { token: "Minus", foreground: vibrantColors.red },
      { token: "Plus", foreground: vibrantColors.green },
      { token: "Arrow", foreground: vibrantColors.blue },
      { token: "FatArrow", foreground: vibrantColors.yellow },
      { token: "Divide", foreground: vibrantColors.purple },
      { token: "Multiply", foreground: vibrantColors.orange },
      { token: "Percent", foreground: vibrantColors.orange },
      { token: "Dollar", foreground: "#888888" },
      { token: "Exclamation", foreground: vibrantColors.red },
      { token: "Question", foreground: vibrantColors.green },
      { token: "Tilde", foreground: vibrantColors.blue },
      { token: "At", foreground: vibrantColors.yellow },
      { token: "Caret", foreground: vibrantColors.purple },
      { token: "Shebang", foreground: vibrantColors.orange },
      { token: "Other", foreground: "#888888", background: "#FFFFFF" },
    ],
    colors: {
      "editor.foreground": isLight ? "#000000" : "#FFFFFF", 
      "editor.background": isLight ? "#F2F2F2" : "#282828", 
      "editorCursor.foreground": isLight ? "#000000" : "#A7A7A7",
      "editor.lineHighlightBackground": isLight ? "#EFEFEF" : "#333333",
      "editorLineNumber.foreground": isLight ? "#AAAAAA" : "#858585",
      "editor.selectionBackground": isLight ? "#D0D0D0" : "#264F78",
      "editor.inactiveSelectionBackground": isLight ? "#E0E0E0" : "#3A3D41",
    },
  };
}


function defineThemes() {
  // Light theme
  monaco.editor.defineTheme("src", {
    base: "vs",
    inherit: true,
    ...commonOptions("light"),
  });

  // Dark theme
  monaco.editor.defineTheme("src-dark", {
    base: "vs-dark",
    inherit: true,
    ...commonOptions("dark"),
  });
}

export default class App {
  readonly #window: Window & monaco.Window & typeof globalThis = self;

  readonly #intoServer: IntoServer = new IntoServer();
  readonly #fromServer: FromServer = FromServer.create();

  initializeMonaco(): void {
    this.#window.MonacoEnvironment = new Environment();
  }

  createModel(client: Client): monaco.editor.ITextModel {
    const language = Language.initialize(client);

    const value = `use { host } from std

effect Make: async + throws + execs + reads + writes {
    catch() [throws]
    await<T>(f: Future<T>) [async, throws] -> T
    exec(arg0: string, args: stringvec) [Make] -> i32
}

struct Local {
    host: host
}

impl Make for Local {
    fn catch(self) [throws] {
    }
    fn await<T>(f: Future<T>) [async, trhows] -> T {
        yield()
    }
    fn exec(self, arg0: string, args: vec<string>) [Vm] -> i32 {
        self.host.read("jobserver")
        if self.host.exec(arg0, args) {
            raise(1)
        }
    }
}`;

    const id = language.id;
    const uri = monaco.Uri.parse("inmemory://exec.src");

    const model = monaco.editor.createModel(value, id, uri);

    // Define and set the custom themes
    defineThemes();
    const theme = "src";
    monaco.editor.setTheme(theme);

    model.onDidChangeContent(
      debounce(() => {
        const text = model.getValue();
        client.notify(proto.DidChangeTextDocumentNotification.type.method, {
          textDocument: {
            version: 0,
            uri: model.uri.toString(),
          },
          contentChanges: [
            {
              range: monacoToProtocol.asRange(model.getFullModelRange()),
              text,
            },
          ],
        } as proto.DidChangeTextDocumentParams);
      }, 200)
    );

    client.pushAfterInitializeHook(async () => {
      client.notify(proto.DidOpenTextDocumentNotification.type.method, {
        textDocument: {
          uri: model.uri.toString(),
          languageId: language.id,
          version: 0,
          text: model.getValue(),
        },
      } as proto.DidOpenTextDocumentParams);
    });

    return model;
  }

  createEditor(client: Client): void {
    document
      .querySelector(".content")
      ?.querySelector("main")
      ?.setAttribute("style", "max-width: 100%;");

    document
      .querySelector(".nav-chapters")
      ?.setAttribute("style", "display: none;");

    const container = document.getElementById("editor")!;

    this.initializeMonaco();
    const model = this.createModel(client);

    monaco.editor.create(container, {
      model,
      automaticLayout: true,
    });
  }

  async run(): Promise<void> {
    const client = new Client(this.#fromServer, this.#intoServer);
    const server = await Server.initialize(this.#intoServer, this.#fromServer);

    this.createEditor(client);
    await Promise.all([server.start(), client.start()]);
  }
}
