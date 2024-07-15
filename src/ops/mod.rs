/// This module contains the srclang ops.
pub mod traversal {
    /// The result of a traversal operation.
    #[derive(Debug, PartialEq, Eq)]
    pub enum Control {
        /// Continue the traversal.
        Continue,
        /// Stop the traversal.
        Break,
    }
}
