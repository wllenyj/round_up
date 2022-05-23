pub fn round_up(bytes: u32, align: u32) -> u32 {
    ((bytes + align - 1) & !(align - 1))
}

pub fn round_up_div(bytes: u32, align: u32) -> u32 {
    ((bytes + align - 1) & !(align - 1)) >> align.trailing_zeros()
}

pub fn div_round_up(n: u32, d: u32) -> u32 {
    ((n + d - 1) / (d)) * d
}

