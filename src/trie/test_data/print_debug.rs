pub(crate) static EXPECTED_FMT: &str = "Trie(
    Trie {
        louds: Louds {
            lbs: Fid {
                byte_vec: [
                    160,
                ],
                bit_len: 5,
                chunks: Chunks {
                    chunks: [
                        Chunk {
                            value: 2,
                            blocks: Blocks {
                                blocks: [
                                    Block {
                                        value: 1,
                                        length: 1,
                                    },
                                    Block {
                                        value: 1,
                                        length: 1,
                                    },
                                    Block {
                                        value: 2,
                                        length: 1,
                                    },
                                    Block {
                                        value: 2,
                                        length: 1,
                                    },
                                ],
                                blocks_cnt: 4,
                            },
                        },
                        Chunk {
                            value: 2,
                            blocks: Blocks {
                                blocks: [
                                    Block {
                                        value: 0,
                                        length: 1,
                                    },
                                ],
                                blocks_cnt: 1,
                            },
                        },
                    ],
                    chunks_cnt: 2,
                },
                table: PopcountTable {
                    bit_length: 1,
                    table: [
                        0,
                        1,
                    ],
                },
            },
        },
        trie_labels: [
            TrieLabel {
                label: 97,
                value: Some(
                    (),
                ),
            },
        ],
    },
)";
