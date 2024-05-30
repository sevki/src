use anyhow::anyhow;
use std::result::Result::Ok;
use std::sync::Arc;
use tower_lsp::{jsonrpc, lsp_types::*, LanguageServer};

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

    lsp::ServerCapabilities {
        text_document_sync,
        document_symbol_provider,
        
        ..Default::default()
    }
}

pub struct Server {
    pub client: tower_lsp::Client,
    pub db: Arc<crate::db::LspServerDatabase>,
}

impl Server {
    pub fn new(client: tower_lsp::Client) -> Self {
        // let session = crate::core::Session::new(Some(client.clone()), language);
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
        // *self.session.client_capabilities.write().await = Some(params.capabilities);
        let capabilities = capabilities();
        Ok(InitializeResult {
            capabilities,
            ..InitializeResult::default()
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

        let typ = lsp::MessageType::INFO;
        let message = format!("opened document: {}", params.text_document.uri.as_str());
        self.client.log_message(typ, message).await;
        let mut errors = vec![];
        let wrapper = srclang::lexer::TripleIterator::new(&params.text_document.text);
        let t = srclang::parser::src::SourceParser::new()
            .parse(&mut errors, wrapper)
            .unwrap();
        let mut color = "rgb(255, 87, 51)";
        if let Ok(channel_syntax) = get_channel_syntax() {
            channel_syntax.set_value(format!("{:#?}", t).as_str());
            if !errors.is_empty() {
                channel_syntax
                    .style()
                    .set_property("background-color", color)
                    .expect("failed to set style");
            }
        }

        web_sys::console::log_1(&"server::did parse".into());
        self.client
            .log_message(typ, format!("{:#?}", params.text_document.text))
            .await;
        // let session = self.session.clone();
        // crate::handler::text_document::did_open(session, params).await.unwrap();
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        web_sys::console::log_1(&"server::did_change".into());
        let typ = lsp::MessageType::INFO;
        let mut errors = vec![];
        let wrapper = srclang::lexer::TripleIterator::new("");

        let t = srclang::parser::src::SourceParser::new().parse(&mut errors, wrapper);
        if errors.is_empty() {
            for change in params.content_changes {
                self.client
                    .log_message(typ, format!("{:#?}", change.text))
                    .await;
            }
        } else {
            self.client.log_message(typ, format!("{:#?}", errors)).await;
        }
    }

    async fn document_symbol(
        &self,
        params: lsp::DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<lsp::DocumentSymbolResponse>> {
        web_sys::console::log_1(&"server::document_symbol".into());
        // let session = self.session.clone();
        // let result = crate::handler::text_document::document_symbol(session, params).await;
        // Ok(result.map_err(crate::core::IntoJsonRpcError)?)
        Ok(None)
    }

    async fn hover(&self, params: lsp::HoverParams) -> jsonrpc::Result<Option<lsp::Hover>> {
        web_sys::console::log_1(&"server::hover".into());
        // let session = self.session.clone();
        // let result = crate::handler::text_document::hover(session, params).await;
        // Ok(result.map_err(crate::core::IntoJsonRpcError)?)
        Ok(None)
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
