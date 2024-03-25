use crate::map;
use super::Trie;

/// A trie builder for [Trie].
pub struct TrieBuilder<Label>(map::TrieBuilder<Label, ()>);

impl<Label: Ord + Clone> TrieBuilder<Label> {
    /// Return a [TrieBuilder].
    pub fn new() -> Self {
        Self(map::TrieBuilder::new())
    }

    /// Add a cloneable entry.
    pub fn push<Arr: AsRef<[Label]>>(&mut self, entry: Arr)
    where
        Label: Clone,
    {
        self.0.push(entry, ());
    }

    /// Add an entry.
    pub fn insert<Arr: IntoIterator<Item = Label>>(&mut self, entry: Arr) {
        self.0.insert(entry, ());
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Label> {
        Trie(self.0.build())
    }
}

impl<Label: Ord + Clone> Default for TrieBuilder<Label> {
    fn default() -> Self {
        Self::new()
    }
}
