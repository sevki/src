use lsp::{InitializeParams, InitializeResult, Url};

use lsp_text::RopeExt;
use salsa::function::DynDb;
use srclang_collections::Map;
use srclang::{
    analyzer::{self, span_text},
    compiler::text::{self, Document, SourceProgram},
    parser::{
        self,
        span::{ByteOrLineColOrCoord, ByteOrLineColOrCoordInterned},
    },
    Db,
};
use std::{borrow::BorrowMut, sync::Mutex};
use tower_lsp::{jsonrpc, LanguageServer};

use crate::update_channel;

pub struct LspServerDatabase {
    db: Mutex<src_lang::analyzer::db::Database>,
}

impl LspServerDatabase {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(src_lang::analyzer::db::Database::default()),
        }
    }
}

// create a global singleton for input files input_files: Map<Url, SourceProgram>,
// so that we can access it from the analyzer it should be a mutable map so that we can add new files to it
// we can use a Mutex to make it thread safe
lazy_static::lazy_static! {
    static ref FILES: Mutex<Map<Url, Document>> = Mutex::new(Map::default());
}
#[tower_lsp::async_trait]
impl LanguageServer for LspServerDatabase {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        web_sys::console::log_1(&"server::initialize".into());
        Ok(InitializeResult {
            ..InitializeResult::default()
        })
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        web_sys::console::log_1(&"server::shutdown".into());
        Ok(())
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        web_sys::console::log_1(&"server::initialized".into());
    }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        let url = params.text_document.uri;
        let text = params.text_document.text;
        let db = &*self.db.lock().unwrap();
        let document = text::Document::new(db, url.to_string(), ropey::Rope::from_str(&text));
        FILES.lock().unwrap().insert(url, document);
        update_channel(&text);
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        let url = params.text_document.uri;
        let db = &*self.db.lock().unwrap();
        let mut files = FILES.lock().unwrap();
        let file = files.get(&url).unwrap();
        let mut rope = file.text(db);
        let text_edit = rope.build_edit(&params.content_changes[0]).unwrap();
        rope.apply_edit(&text_edit);
        let text = rope.clone().to_string();
        files.insert(url.clone(), text::Document::new(db, url.to_string(), rope));
        update_channel(&text);
    }

    async fn document_symbol(
        &self,
        params: lsp::DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<lsp::DocumentSymbolResponse>> {
        web_sys::console::log_1(&"server::document_symbol".into());
        let db = &*self.db.lock().unwrap();

        Ok(None)
    }

    async fn hover(&self, params: lsp::HoverParams) -> jsonrpc::Result<Option<lsp::Hover>> {
        web_sys::console::log_1(&"server::hover".into());
        let db = &*self.db.lock().unwrap();
        let url = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let files = FILES.lock().unwrap();
        let text = files.get(&url).unwrap();
        let text = text.text(db);
        let text = text::SourceProgram::new(db, url.to_string(), text.to_string());
        let line: usize = position.line.try_into().unwrap();
        let character: usize = position.character.try_into().unwrap();
        web_sys::console::log_1(&format!("line: {}, character: {}", line, character).into());
        let spanned = analyzer::get_symbol(
            db,
            text,
            ByteOrLineColOrCoordInterned::new(db, ByteOrLineColOrCoord::LineCol(line, character)),
        );

        web_sys::console::log_1(&format!("{:#?}", spanned).into());
        let hover = spanned.map(|span| {
            // let text = span_text(db, span);
            lsp::Hover {
                contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                    kind: lsp::MarkupKind::Markdown,
                    value: format!("```src\n{:#?}\n```src anaylzer", span),
                }),
                range: None,
            }
        });
        Ok(hover)
    }

    async fn document_highlight(
        &self,
        params: lsp::DocumentHighlightParams,
    ) -> jsonrpc::Result<Option<Vec<lsp::DocumentHighlight>>> {
        web_sys::console::log_1(&"server::document_highlight".into());
        let db = &*self.db.lock().unwrap();
        let url = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let text = FILES.lock().unwrap();
        let text = text.get(&url).unwrap();
        let text = text.text(db);
        let text = text::SourceProgram::new(db, url.to_string(), text.to_string());
        let text = analyzer::get_symbol(
            db,
            text,
            ByteOrLineColOrCoordInterned::new(
                db,
                ByteOrLineColOrCoord::LineCol(position.line as usize, position.character as usize),
            ),
        );
        let hightlight_kind = lsp::DocumentHighlightKind::TEXT;
        let highlights = text.map(|text| {
            vec![lsp::DocumentHighlight {
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 0,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 0,
                    },
                },
                kind: Some(hightlight_kind),
            }]
        });
        Ok(highlights)
    }
}
