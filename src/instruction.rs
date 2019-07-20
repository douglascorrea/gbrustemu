#[derive(Debug)]
pub enum Instruction {
    Nop,
    LdBc(u16),
    LdDe(u16),
    LdHl(u16),
    LdSp(u16),
    XorA,
    XorB,
    XorC,
    XorD,
    XorE,
    XorH,
    XorL,
    XorHl,
    Xor(u8),
    LddHlA,
    BitbA(u8),
    BitbB(u8),
    BitbC(u8),
    BitbD(u8),
    BitbE(u8),
    BitbH(u8),
    BitbHL(u8),
}
