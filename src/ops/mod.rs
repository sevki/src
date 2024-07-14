/// This module contains the srclang ops.
pub mod traversal {
    /// The result of a traversal operation.
    pub enum Result {
        /// Continue the traversal.
        Continue,
        /// Stop the traversal.
        Break,
    }
}
