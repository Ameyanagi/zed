use std::{num::NonZeroU32, sync::Arc};

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[Option<HighlightId>]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightId(NonZeroU32);

impl HighlightId {
    pub fn new(capture_id: u32) ->  Self {
        Self(NonZeroU32::new(capture_id + 1).unwrap_or(NonZeroU32::MAX))
    }
}

impl HighlightMap {
    #[inline]
    pub fn from_ids(highlight_ids: impl IntoIterator<Item = Option<HighlightId>>) -> Self {
        Self(highlight_ids.into_iter().collect())
    }

    #[inline]
    pub fn get(&self, capture_id: u32) -> Option<HighlightId> {
        self.0.get(capture_id as usize).copied().flatten()
    }
}

impl HighlightId {
    pub const TABSTOP_INSERT_ID: HighlightId =
        unsafe { HighlightId(NonZeroU32::new_unchecked(u32::MAX - 1)) };
    pub const TABSTOP_REPLACE_ID: HighlightId =
        unsafe { HighlightId(NonZeroU32::new_unchecked(u32::MAX - 2)) };
}

impl Default for HighlightMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}

impl From<HighlightId> for usize {
    fn from(value: HighlightId) -> Self {
        value.0.get() as usize - 1
    }
}
