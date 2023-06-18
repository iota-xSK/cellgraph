use crate::automaton::Automaton;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SavedState {
    pub automaton: Automaton,
    pub code: String,
}
