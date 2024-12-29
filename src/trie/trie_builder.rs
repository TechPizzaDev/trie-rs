use super::Trie;
use crate::map;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie builder for [Trie].
pub struct TrieBuilder<Label>(map::TrieBuilder<Label, ()>);

impl<Label: Ord> TrieBuilder<Label> {
    /// Return a [TrieBuilder].
    pub fn new() -> Self {
        Self(map::TrieBuilder::new())
    }

    /// Insert the given sequence.
    pub fn insert<Key: IntoIterator<Item = Label>>(&mut self, key: Key) -> bool {
        self.0.entry(key).replace(()).is_some()
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Label> {
        Trie(self.0.build())
    }
}

impl<Label: Ord> Default for TrieBuilder<Label> {
    fn default() -> Self {
        Self::new()
    }
}
