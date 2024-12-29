use crate::internal_data_structure::naive_trie::NaiveTrie;
use crate::map::{Trie, TrieBuilder};
use fid::bit_vec;
use louds::Louds;

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

    /// Gets the entry for the given sequence for in-place manipulation.
    pub fn entry<Key: IntoIterator<Item = Label>>(
        &mut self,
        key: Key,
    ) -> &mut Option<Value> {
        self.naive_trie.entry(key.into_iter())
    }

    /// Insert a value for the given sequence.
    pub fn insert<Key: IntoIterator<Item = Label>>(
        &mut self,
        key: Key,
        value: Value,
    ) -> Option<Value> {
        self.entry(key).replace(value)
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Label, Value> {
        let mut louds_bits = bit_vec![true, false];
        let mut labels: Vec<Label> = vec![];
        let mut terminals = bit_vec![false, false];
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
                    terminals.push(is_terminal);
                }
                NaiveTrie::PhantomSibling => {
                    louds_bits.push(false);
                }
            }
        }

        louds_bits.shrink_to_fit();
        labels.shrink_to_fit();
        terminals.shrink_to_fit();
        values.shrink_to_fit();

        let louds = if cfg!(debug_assertions) {
            Louds::new(louds_bits).unwrap()
        } else {
            // SAFETY: NaiveTrie should produce valid bits
            unsafe { Louds::new_unchecked(louds_bits) }
        };

        Trie {
            louds,
            labels,
            terminals,
            values,
        }
    }
}
