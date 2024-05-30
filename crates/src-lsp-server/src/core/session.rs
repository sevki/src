use anyhow::anyhow;
use async_lock::{Mutex, RwLock};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use srclang::compiler::ir;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionResourceKind {
    Document,
    Parser,
    Tree,
}

pub struct Session {
    pub server_capabilities: RwLock<lsp::ServerCapabilities>,
    pub client_capabilities: RwLock<Option<lsp::ClientCapabilities>>,
    pub db: RwLock<Option<srclang::compiler::db::Database>>,
    client: Option<tower_lsp::Client>,
}

impl Session {
    pub fn new(client: Option<tower_lsp::Client>) -> Arc<Self> {
        let server_capabilities = RwLock::new(crate::server::capabilities());
        let client_capabilities = Default::default();
        Arc::new(Session {
            db: Default::default(),
            server_capabilities,
            client_capabilities,
            client,
        })
    }

    pub fn client(&self) -> anyhow::Result<&tower_lsp::Client> {
        self.client
            .as_ref()
            .ok_or_else(|| crate::core::Error::ClientNotInitialized.into())
    }

    pub fn insert_document(&self, uri: lsp::Url, document: crate::core::Document) -> anyhow::Result<()> {

        Ok(())
    }

    pub fn remove_document(&self, uri: &lsp::Url) -> anyhow::Result<()> {
       
        Ok(())
    }

    pub async fn semantic_tokens_legend(&self) -> Option<lsp::SemanticTokensLegend> {
        let capabilities = self.server_capabilities.read().await;
        if let Some(capabilities) = &capabilities.semantic_tokens_provider {
            match capabilities {
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options) => Some(options.legend.clone()),
                lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(options) => {
                    Some(options.semantic_tokens_options.legend.clone())
                },
            }
        } else {
            None
        }
    }

    pub async fn get_text(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, crate::core::Text>> {
        Err({
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_text(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, crate::core::Text>> {
        Err({
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_parser(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<ir::Program>>> {
        Err({
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_tree(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Mutex<ir::Program>>> {
        Err({
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_tree(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<ir::Program>>> {
        Err({
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
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