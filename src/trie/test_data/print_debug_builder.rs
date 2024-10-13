pub(crate) static EXPECTED_FMT: &str = "TrieBuilder(
    TrieBuilder {
        naive_trie: Root(
            NaiveTrieRoot {
                children: [
                    IntermOrLeaf(
                        NaiveTrieIntermOrLeaf {
                            children: [
                                IntermOrLeaf(
                                    NaiveTrieIntermOrLeaf {
                                        children: [
                                            IntermOrLeaf(
                                                NaiveTrieIntermOrLeaf {
                                                    children: [],
                                                    label: 112,
                                                    value: Some(
                                                        (),
                                                    ),
                                                },
                                            ),
                                        ],
                                        label: 112,
                                        value: None,
                                    },
                                ),
                            ],
                            label: 97,
                            value: Some(
                                (),
                            ),
                        },
                    ),
                ],
            },
        ),
    },
)";
