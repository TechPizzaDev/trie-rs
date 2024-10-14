use crate::internal_data_structure::naive_trie::NaiveTrie;
use crate::map::{Trie, TrieBuilder};
use fid_rs::Fid;
use louds_rs::Louds;

impl<Label: Ord, Value> Default for TrieBuilder<Label, Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Label: Ord, Value> TrieBuilder<Label, Value> {
    /// Return a [TrieBuilder].
    pub fn new() -> Self {
        let naive_trie = NaiveTrie::make_root();
        Self { naive_trie }
    }

    /// Add a cloneable entry and value.
    pub fn push<Arr: AsRef<[Label]>>(&mut self, entry: Arr, value: Value)
    where
        Label: Clone,
    {
        self.naive_trie.push(entry.as_ref().iter().cloned(), value);
    }

    /// Add an entry and value.
    pub fn insert<Arr: IntoIterator<Item = Label>>(&mut self, entry: Arr, value: Value) {
        self.naive_trie.push(entry.into_iter(), value);
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Label, Value> {
        let mut louds_bits: Vec<bool> = vec![true, false];
        let mut labels: Vec<Label> = vec![];
        let mut terminal_bits: Vec<bool> = vec![false, false];
        let mut values: Vec<Value> = vec![];

        for node in self.naive_trie.into_iter() {
            match node {
                NaiveTrie::Root(_) => {}
                NaiveTrie::IntermOrLeaf(n) => {
                    louds_bits.push(true);
                    labels.push(n.label);

                    let is_terminal = if let Some(value) = n.value {
                        values.push(value);
                        true
                    } else {
                        false
                    };
                    terminal_bits.push(is_terminal);
                }
                NaiveTrie::PhantomSibling => {
                    louds_bits.push(false);
                }
            }
        }

        let louds = Louds::from(louds_bits.as_slice());
        let terminals = Fid::from(terminal_bits.as_slice());

        Trie {
            louds,
            labels,
            terminals,
            values,
        }
    }
}
