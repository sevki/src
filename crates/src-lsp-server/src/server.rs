use anyhow::anyhow;
use src_lang::parser;
use std::result::Result::Ok;
use std::sync::Arc;
use tower_lsp::{jsonrpc, lsp_types::*, LanguageServer};
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;

pub fn capabilities() -> lsp::ServerCapabilities {
    let document_symbol_provider = Some(lsp::OneOf::Left(true));

    let text_document_sync = {
        let options = lsp::TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(lsp::TextDocumentSyncKind::FULL),
            ..Default::default()
        };
        Some(lsp::TextDocumentSyncCapability::Options(options))
    };

    let hover_provider = Some(true.into());

    let semantic_tokens_provider = Some(
        lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(lsp::SemanticTokensOptions {
            legend: lsp::SemanticTokensLegend {
                token_types: vec![],
                token_modifiers: vec![],
            },
            range: Some(true),
            full: Some(lsp::SemanticTokensFullOptions::Delta { delta: Some(true) }),
            ..Default::default()
        }),
    );

    let _document_highlight_provider: Option<OneOf<bool, DocumentHighlightOptions>> =
        Some(lsp::OneOf::Left(true));

    let code_lens_provider = Some(lsp::CodeLensOptions {
        resolve_provider: Some(true),
    });

    let completion_provider = Some(CompletionOptions {
        resolve_provider: Some(true),
        trigger_characters: Some(vec![".".into()]),
        work_done_progress_options: WorkDoneProgressOptions {
            work_done_progress: None,
        },
        all_commit_characters: None,
    });

    lsp::ServerCapabilities {
        text_document_sync,
        document_symbol_provider,
        hover_provider,
        semantic_tokens_provider,
        // document_highlight_provider,
        code_lens_provider,
        completion_provider,
        ..Default::default()
    }
}

pub struct Server {
    pub client: tower_lsp::Client,
    pub db: Arc<crate::db::LspServerDatabase>,
}

impl Server {
    pub fn new(client: tower_lsp::Client) -> Self {
        Server {
            client,
            db: Arc::new(crate::db::LspServerDatabase::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        web_sys::console::log_1(&"server::initialize".into());
        let capabilities = capabilities();
        Ok(InitializeResult {
            server_info: Some(
                ServerInfo {
                    name: "src language server".to_string(),
                    version: Some(env!("CARGO_PKG_VERSION").to_string()),
                }
                .into(),
            ),
            capabilities,
        })
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        web_sys::console::log_1(&"server::initialized".into());
        let typ = lsp::MessageType::INFO;
        let message = "src language server initialized!";
        self.client.log_message(typ, message).await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        web_sys::console::log_1(&"server::shutdown".into());
        Ok(())
    }

    // FIXME: for some reason this doesn't trigger
    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        web_sys::console::log_1(&"server::did_open".into());
        self.db.did_open(params).await;
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        web_sys::console::log_1(&"server::did_change".into());
        self.db.did_change(params).await;
    }

    async fn document_symbol(
        &self,
        params: lsp::DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<lsp::DocumentSymbolResponse>> {
        web_sys::console::log_1(&"server::document_symbol".into());
        self.db.document_symbol(params).await
    }

    async fn hover(&self, params: lsp::HoverParams) -> jsonrpc::Result<Option<lsp::Hover>> {
        web_sys::console::log_1(&"server::hover".into());
        self.db.hover(params).await
    }

    async fn document_highlight(
        &self,
        params: lsp::DocumentHighlightParams,
    ) -> jsonrpc::Result<Option<Vec<lsp::DocumentHighlight>>> {
        web_sys::console::log_1(&"server::document_highlight".into());
        self.db.document_highlight(params).await
    }
}

pub fn get_channel_syntax() -> anyhow::Result<web_sys::HtmlTextAreaElement> {
    use wasm_bindgen::JsCast;
    let element_id = "channel-syntax";
    let channel_syntax = web_sys::window()
        .ok_or_else(|| anyhow!("failed to get window"))?
        .document()
        .ok_or_else(|| anyhow!("failed to get document"))?
        .get_element_by_id(element_id)
        .ok_or_else(|| anyhow!("failed to get channel-syntax element"))?
        .unchecked_into();
    Ok(channel_syntax)
}

pub fn update_channel(input: &str) {
    // let tree = parser::parse(input);
    // // assume errors; use red
    // let element_id = "channel-syntax";
    // match tree {
    //     Ok(module) => {
    //         // use green
    //         web_sys::window()
    //             .unwrap()
    //             .document()
    //             .unwrap()
    //             .get_element_by_id(element_id)
    //             .unwrap()
    //             .set_inner_html(&format!("{:#?}", module));
    //         // set the border of the textarea to green
    //         web_sys::window()
    //             .unwrap()
    //             .document()
    //             .unwrap()
    //             .get_element_by_id(element_id)
    //             .unwrap()
    //             .unchecked_into::<HtmlTextAreaElement>()
    //             .style()
    //             .set_property("border", "1px solid green")
    //             .unwrap();
    //     }
    //     Err(errs) => {
    //         // use red
    //         web_sys::window()
    //             .unwrap()
    //             .document()
    //             .unwrap()
    //             .get_element_by_id(element_id)
    //             .unwrap()
    //             .set_inner_html(&format!("{:#?}", errs));
    //         // set the border of the textarea to red
    //         web_sys::window()
    //             .unwrap()
    //             .document()
    //             .unwrap()
    //             .get_element_by_id(element_id)
    //             .unwrap()
    //             .unchecked_into::<HtmlTextAreaElement>()
    //             .style()
    //             .set_property("border", "1px solid red")
    //             .unwrap();
    //     }
    // };
}
