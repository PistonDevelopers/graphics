use {BackEnd};

/// Implemented by contexts that can clear the background.
pub trait Clear {
    /// Clears the background.
    fn clear<B: BackEnd>(back_end: &mut BackEnd);
}

