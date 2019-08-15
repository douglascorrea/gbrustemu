#[derive(Debug)]
pub enum Instruction {
    Nop,
    LdBc(u16),
    LdDe(u16),
    LdHl(u16),
    LdHln(u8),
    LdSp(u16),
    LdA(u8),
    LdB(u8),
    LdC(u8),
    LdD(u8),
    LdE(u8),
    LdH(u8),
    LdL(u8),
    LdAa,
    LdBa,
    LdCa,
    LdDa,
    LdEa,
    LdHa,
    LdLa,
    LdBcA,
    LdDeA,
    LdHlA,
    LdXxA(u16),
    LdFf00U8a(u8),
    LdAFf00U8(u8),
    LdFf00Ca,
    LddHlA,
    LdiHlA,
    LdiAHl,
    LdABc,
    LdADe,
    LdAb,
    LdAc,
    LdAd,
    LdAe,
    LdAh,
    LdAl,
    XorA,
    XorB,
    XorC,
    XorD,
    XorE,
    XorH,
    XorL,
    XorHl,
    Xor(u8),
    BitbA(u8),
    BitbB(u8),
    BitbC(u8),
    BitbD(u8),
    BitbE(u8),
    BitbH(u8),
    BitbL(u8),
    BitbHL(u8),
    Jr(i8),
    JrNz(i8),
    JrZ(i8),
    JrNc(i8),
    JrC(i8),
    Jp(u16),
    IncA,
    IncB,
    IncC,
    IncD,
    IncE,
    IncH,
    IncL,
    IncBc,
    IncDe,
    IncHl,
    IncSp,
    IncHlNoflags,
    Call(u16),
    CallNz(u16),
    CallZ(u16),
    CallNc(u16),
    CallC(u16),
    PushAf,
    PushBc,
    PushDe,
    PushHl,
    PopAf,
    PopBc,
    PopDe,
    PopHl,
    RlA,
    RlB,
    RlC,
    RlD,
    RlE,
    RlH,
    RlL,
    RlHl,
    RLA,
    DecA,
    DecB,
    DecC,
    DecD,
    DecE,
    DecH,
    DecL,
    DecHl,
    SubA,
    SubB,
    SubC,
    SubD,
    SubE,
    SubH,
    SubL,
    SubHl,
    Sub(u8),
    AddAa,
    AddAb,
    AddAc,
    AddAd,
    AddAe,
    AddAh,
    AddAl,
    AddAhl,
    AddA(u8),
    Ret,
    CpA,
    CpB,
    CpC,
    CpD,
    CpE,
    CpH,
    CpL,
    CpHl,
    Cp(u8),
    Di,
    Ei,
}
