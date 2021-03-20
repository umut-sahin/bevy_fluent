/// State
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum State {
    LoadAssets,
    Snapshot,
    Done,
}