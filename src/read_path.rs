/// Read features from the in-process hot tier first; fall back to Redis only for cold cards.
pub enum Source { Hot, Cold }
pub fn pick(is_hot: bool) -> Source { if is_hot { Source::Hot } else { Source::Cold } }
