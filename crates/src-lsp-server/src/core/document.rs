use async_lock::Mutex;
use lsp_text::RopeExt;
use std::sync::Arc;

use srclang::{compiler::ir, Db};


pub struct Document {
    pub content: ropey::Rope,
    pub db: &'static dyn Db,
}

impl Document {
    pub async fn open(
        session: Arc<crate::core::Session>,
        params: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<Option<Self>> {
        // let mut parser = crate::core::parser::javascript(&session.language)?;
        // let content = ropey::Rope::from(params.text_document.text);
        // let result = {
        //     let content = content.clone();
        //     let byte_idx = 0;
        //     let callback = content.chunk_walker(byte_idx).callback_adapter_for_tree_sitter();
        //     let old_tree = None;
        //     parser.parse_with(callback, old_tree)?
        // };
        // crate::core::syntax::update_channel(result.as_ref());
        Ok(None)
    }

    pub async fn change<'changes>(
        session: Arc<crate::core::Session>,
        uri: &lsp::Url,
        content: &ropey::Rope,
    ) -> anyhow::Result<Option<ir::Program>> {
        let result = {
            let parser = session.get_mut_parser(uri).await?;
            let mut parser = parser.lock().await;
            let text = content.chunks().collect::<String>();
            // parser.parse(text, None)?
        };
        // crate::core::syntax::update_channel(result.as_ref());
        // if let Some(tree) = result {
        //     {
        //         let tree = tree.clone();
        //         *session.get_mut_tree(uri).await?.value_mut() = Mutex::new(tree);
        //     }
        //     Ok(Some(tree))
        // } else {
            Ok(None)
        // }
    }

    /// Return the language-id and textual content portion of the [`Document`].
    pub fn text(&self) -> crate::core::Text {
        crate::core::Text {
            content: self.content.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DocumentState {
    Closed,
    Opened,
}
