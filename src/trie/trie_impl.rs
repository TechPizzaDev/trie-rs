use crate::inc_search::IncSearch;
use crate::iter::{Keys, KeysExt, PostfixIter, PrefixIter, SearchIter};
use crate::map;
use crate::try_collect::TryFromIterator;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie for sequences of the type `Label`.
pub struct Trie<Label>(pub map::Trie<Label, ()>);

impl<Label: Ord> Trie<Label> {
    /// Return true if `query` is an exact match.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    ///
    /// # Examples
    /// In the following example we illustrate how to query an exact match.
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let trie = Trie::from_iter(["a", "app", "apple", "better", "application"]
    ///     .into_iter()
    ///     .map(|s| s.bytes())
    /// );
    /// assert!(trie.exact_match("application"));
    /// assert!(trie.exact_match("app"));
    /// assert!(!trie.exact_match("appla"));
    ///
    /// ```
    pub fn exact_match(&self, query: impl AsRef<[Label]>) -> bool {
        self.0.exact_match(query).is_some()
    }

    /// Return the common prefixes of `query`.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    ///
    /// # Examples
    /// In the following example we illustrate how to query the common prefixes of a query string.
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let trie = Trie::from_iter(["a", "app", "apple", "better", "application"]
    ///     .into_iter()
    ///     .map(|s| s.bytes())
    /// );
    /// let results: Vec<String> = trie.common_prefix_search("application").collect();
    /// assert_eq!(results, vec!["a", "app", "application"]);
    /// ```
    pub fn common_prefix_search<C, M>(
        &self,
        query: impl AsRef<[Label]>,
    ) -> Keys<PrefixIter<'_, Label, (), C, M>>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        // TODO: We could return Keys iterators instead of collecting.
        self.0.common_prefix_search(query).keys()
    }

    /// Return all entries that match `query`.
    pub fn predictive_search<C, M>(
        &self,
        query: impl AsRef<[Label]>,
    ) -> Keys<SearchIter<'_, Label, (), C, M>>
    where
        C: TryFromIterator<Label, M> + Clone,
        Label: Clone,
    {
        self.0.predictive_search(query).keys()
    }

    /// Return the postfixes of all entries that match `query`.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    ///
    /// # Examples
    /// In the following example we illustrate how to query the postfixes of a query string.
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let trie = Trie::from_iter(["a", "app", "apple", "better", "application"]
    ///     .into_iter()
    ///     .map(|s| s.bytes())
    /// );
    ///
    /// let results: Vec<String> = trie.postfix_search("application").collect();
    /// assert!(results.is_empty());
    ///
    /// let results: Vec<String> = trie.postfix_search("app").collect();
    /// assert_eq!(results, vec!["le", "lication"]);
    ///
    /// ```
    pub fn postfix_search<C, M>(
        &self,
        query: impl AsRef<[Label]>,
    ) -> Keys<PostfixIter<'_, Label, (), C, M>>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        self.0.postfix_search(query).keys()
    }

    /// Returns an iterator across all keys in the trie.
    ///
    /// # Examples
    /// In the following example we illustrate how to iterate over all keys in the trie.
    /// Note that the order of the keys is not guaranteed, as they will be returned in
    /// lexicographical order.
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let trie = Trie::from_iter(["a", "app", "apple", "better", "application"]
    ///     .into_iter()
    ///     .map(|s| s.bytes())
    /// );
    ///
    /// let results: Vec<String> = trie.iter().collect();
    /// assert_eq!(results, vec!["a", "app", "apple", "application", "better"]);
    /// ```
    pub fn iter<C, M>(&self) -> Keys<PostfixIter<'_, Label, (), C, M>>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        self.postfix_search([])
    }

    /// Create an incremental search. Useful for interactive applications. See
    /// [crate::inc_search] for details.
    pub fn inc_search(&self) -> IncSearch<'_, Label, ()> {
        IncSearch::new(&self.0)
    }

    /// Return true if `query` is a prefix.
    ///
    /// Note: A prefix may be an exact match or not, and an exact match may be a
    /// prefix or not.
    pub fn is_prefix(&self, query: impl AsRef<[Label]>) -> bool {
        self.0.is_prefix(query)
    }

    /// Return the longest shared prefix of `query`.
    pub fn longest_prefix<C, M>(&self, query: impl AsRef<[Label]>) -> Option<C>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        self.0.longest_prefix(query)
    }
}

impl<Label, Key> FromIterator<Key> for Trie<Label>
where
    Key: IntoIterator<Item = Label>,
    Label: Ord + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        Self: Sized,
        T: IntoIterator<Item = Key>,
    {
        let mut builder = super::TrieBuilder::new();
        for key in iter {
            builder.insert(key);
        }
        builder.build()
    }
}

#[cfg(test)]
mod search_tests {
    use crate::{
        trie::test_data::{print_debug, print_debug_builder},
        Trie, TrieBuilder,
    };
    use std::iter::FromIterator;

    fn build_trie() -> Trie<u8> {
        let mut builder = TrieBuilder::new();
        builder.insert("a".bytes());
        builder.insert("app".bytes());
        builder.insert("apple".bytes());
        builder.insert("better".bytes());
        builder.insert("application".bytes());
        builder.insert("アップル🍎".bytes());
        let trie = builder.build();
        trie
    }

    #[test]
    fn trie_from_iter() {
        let trie = Trie::<u8>::from_iter(
            ["a", "app", "apple", "better", "application"]
                .into_iter()
                .map(|s| s.bytes()),
        );
        assert!(trie.exact_match("application"));
    }

    #[test]
    fn collect_a_trie() {
        let trie: Trie<u8> = ["a", "app", "apple", "better", "application"]
            .into_iter()
            .map(|s| s.bytes())
            .collect();
        assert!(trie.exact_match("application"));
    }

    #[test]
    fn clone() {
        let trie = build_trie();
        let _c: Trie<u8> = trie.clone();
    }

    #[rustfmt::skip]
    #[test]
    fn print_debug() {
        let trie: Trie<u8> = ["a".bytes()].into_iter().collect();
        let actual = format!("{:#?}", trie);
        assert_eq!(actual, print_debug::EXPECTED_FMT);
    }

    #[rustfmt::skip]
    #[test]
    fn print_debug_builder() {
        let mut builder = TrieBuilder::new();
        builder.insert("a".bytes());
        builder.insert("app".bytes());
        let actual = format!("{:#?}", builder);
        assert_eq!(actual, print_debug_builder::EXPECTED_FMT);
    }

    #[test]
    fn use_empty_queries() {
        let trie = build_trie();
        assert!(!trie.exact_match(""));
        let _ = trie.predictive_search::<String, _>("").next();
        let _ = trie.postfix_search::<String, _>("").next();
        let _ = trie.common_prefix_search::<String, _>("").next();
    }

    #[cfg(feature = "mem_dbg")]
    #[test]
    /// ```sh
    /// cargo test --features mem_dbg memsize -- --nocapture
    /// ```
    fn memsize() {
        use mem_dbg::*;
        use std::{
            env,
            fs::File,
            io::{BufRead, BufReader},
        };

        const COUNT: usize = 100;
        let mut builder = TrieBuilder::new();

        let repo_root = env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable must be set.");
        let edict2_path = format!("{}/benches/edict.furigana", repo_root);
        println!("Reading dictionary file from: {}", edict2_path);

        let mut n_words = 0;
        let mut accum = 0;
        for result in BufReader::new(File::open(edict2_path).unwrap())
            .lines()
            .take(COUNT)
        {
            let key = result.unwrap();
            accum += key.len();
            builder.insert(key.bytes());
            n_words += 1;
        }
        println!("Read {} words, {} bytes.", n_words, accum);

        let trie = builder.build();
        let trie_size = trie.mem_size(SizeFlags::default());
        eprintln!("Trie size {trie_size}");
        let uncompressed: Vec<String> = trie.iter().collect();
        let uncompressed_size = uncompressed.mem_size(SizeFlags::default());
        eprintln!("Uncompressed size {}", uncompressed_size);
        assert!(accum < trie_size); // This seems wrong to me.
        assert!(trie_size < uncompressed_size);
    }

    mod exact_match_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.exact_match(query);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", true),
            t2: ("app", true),
            t3: ("apple", true),
            t4: ("application", true),
            t5: ("better", true),
            t6: ("アップル🍎", true),
            t7: ("appl", false),
            t8: ("appler", false),
        }
    }

    mod is_prefix_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.is_prefix(query);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", true),
            t2: ("app", true),
            t3: ("apple", false),
            t4: ("application", false),
            t5: ("better", false),
            t6: ("アップル🍎", false),
            t7: ("appl", true),
            t8: ("appler", false),
            t9: ("アップル", true),
            t10: ("ed", false),
            t11: ("e", false),
            t12: ("", true),
        }
    }

    mod predictive_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<String> = trie.predictive_search(query).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec!["a", "app", "apple", "application"]),
            t2: ("app", vec!["app", "apple", "application"]),
            t3: ("appl", vec!["apple", "application"]),
            t4: ("apple", vec!["apple"]),
            t5: ("b", vec!["better"]),
            t6: ("c", Vec::<&str>::new()),
            t7: ("アップ", vec!["アップル🍎"]),
        }
    }

    mod common_prefix_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<String> = trie.common_prefix_search(query).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec!["a"]),
            t2: ("ap", vec!["a"]),
            t3: ("appl", vec!["a", "app"]),
            t4: ("appler", vec!["a", "app", "apple"]),
            t5: ("bette", Vec::<&str>::new()),
            t6: ("betterment", vec!["better"]),
            t7: ("c", Vec::<&str>::new()),
            t8: ("アップル🍎🍏", vec!["アップル🍎"]),
        }
    }
}
