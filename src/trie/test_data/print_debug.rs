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
        labels: [
            97,
        ],
        terminals: Fid {
            byte_vec: [
                32,
            ],
            bit_len: 3,
            chunks: Chunks {
                chunks: [
                    Chunk {
                        value: 0,
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
                    Chunk {
                        value: 0,
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
                    Chunk {
                        value: 1,
                        blocks: Blocks {
                            blocks: [
                                Block {
                                    value: 1,
                                    length: 1,
                                },
                            ],
                            blocks_cnt: 1,
                        },
                    },
                ],
                chunks_cnt: 3,
            },
            table: PopcountTable {
                bit_length: 1,
                table: [
                    0,
                    1,
                ],
            },
        },
        values: [
            (),
        ],
    },
)";
