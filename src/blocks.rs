#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]

pub(crate) const BITMAPS_HALFS: &[(u32, char)] = &[(0x00000000, ' '), (0x0000ffff, '▄')];

pub(crate) const BITMAPS_BLOCKS: &[(u32, char)] = &[
    (0x00000000, ' '),
    (0x0000000f, '▁'),
    (0x000000ff, '▂'),
    (0x00000fff, '▃'),
    (0x0000ffff, '▄'),
    (0x000fffff, '▅'),
    (0x00ffffff, '▆'),
    (0x0fffffff, '▇'),
];

pub(crate) const BITMAPS_NO_SLOPES: &[(u32, char)] = &[
    (0x00000000, ' '),
    (0x0000000f, '▁'),
    (0x000000ff, '▂'),
    (0x00000fff, '▃'),
    (0x0000ffff, '▄'),
    (0x000fffff, '▅'),
    (0x00ffffff, '▆'),
    (0x0fffffff, '▇'),
    (0xeeeeeeee, '▊'),
    (0xcccccccc, '▌'),
    (0x88888888, '▎'),
    (0x0000cccc, '▖'),
    (0x00003333, '▗'),
    (0xcccc0000, '▘'),
    (0xcccc3333, '▚'),
    (0x33330000, '▝'),
    (0x000ff000, '━'),
    (0x66666666, '┃'),
    (0x00077666, '┏'),
    (0x000ee666, '┓'),
    (0x66677000, '┗'),
    (0x666ee000, '┛'),
    (0x66677666, '┣'),
    (0x666ee666, '┫'),
    (0x000ff666, '┳'),
    (0x666ff000, '┻'),
    (0x666ff666, '╋'),
    (0x000cc000, '╸'),
    (0x00066000, '╹'),
    (0x00033000, '╺'),
    (0x00066000, '╻'),
    (0x06600660, '╏'),
    (0x000f0000, '─'),
    (0x0000f000, '─'),
    (0x44444444, '│'),
    (0x22222222, '│'),
    (0x000e0000, '╴'),
    (0x0000e000, '╴'),
    (0x44440000, '╵'),
    (0x22220000, '╵'),
    (0x00030000, '╶'),
    (0x00003000, '╶'),
    (0x00004444, '╵'),
    (0x00002222, '╵'),
    (0x44444444, '⎢'),
    (0x22222222, '⎥'),
    (0x0f000000, '⎺'),
    (0x00f00000, '⎻'),
    (0x00000f00, '⎼'),
    (0x000000f0, '⎽'),
    (0x00066000, '▪'),
];
