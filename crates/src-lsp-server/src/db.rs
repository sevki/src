use std::sync::{Arc, Mutex};
use anyhow::Result;
use lsp::{InitializeParams, InitializeResult, Url};
use salsa::ParallelDatabase;
use src_collections::Map;
use srclang::compiler::text::SourceProgram;
use tower_lsp::{jsonrpc, LanguageServer};


pub struct LspServerDatabase {
    db: Mutex<srclang::compiler::db::Database>,
    input_files: Map<Url, SourceProgram>,
}

impl LspServerDatabase {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(srclang::compiler::db::Database::default()),
            input_files: Map::default(),
        }
    }

    pub fn db(&self) -> std::sync::MutexGuard<srclang::compiler::db::Database> {
        self.db.lock().unwrap()
    }

    pub fn input_files(&self) -> &Map<Url, SourceProgram> {
        &self.input_files
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LspServerDatabase {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        web_sys::console::log_1(&"server::initialize".into());
        // *self.session.client_capabilities.write().await = Some(params.capabilities);
        Ok(InitializeResult {
            ..InitializeResult::default()
        })
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        web_sys::console::log_1(&"server::shutdown".into());
        Ok(())
    }
}
