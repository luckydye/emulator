
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    BC,
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    HLI,
    SP,
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    HLI,
    D8,
    D16,
}

pub enum LoadType {
    Byte(LoadByteSource, LoadByteTarget),
    Word(LoadByteSource, LoadByteTarget),
    AFromIndirect(u8),
    IndirectFromA(u8),
    AFromByteAddress(u8),
    ByteAddressFromA(u8),
}

pub enum StackTarget {
    BC,
    HL,
    DE
}
