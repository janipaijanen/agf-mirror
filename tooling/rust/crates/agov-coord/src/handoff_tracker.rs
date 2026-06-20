use crate::coord_validator::CoordMessage;

/// A handoff message that has not yet been acknowledged or resolved.
#[derive(Debug, Clone)]
pub struct PendingHandoff {
    pub id: String,
    pub from: String,
    pub to: String,
    pub topic: String,
}

/// Find all pending (unresolved) handoffs in a set of messages.
///
/// Phase 2 command: backing logic for `agov handoff next`.
pub fn find_pending(messages: &[CoordMessage]) -> Vec<PendingHandoff> {
    todo!()
}
