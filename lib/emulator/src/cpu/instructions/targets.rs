
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
    HLI,
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadByteTarget, LoadByteSource),
    AFromIndirect(u8),
    IndirectFromA(u8),
    AFromByteAddress(u8),
    ByteAddressFromA(u8),
    BytesToSP(),
}

pub enum StackTarget {
    BC,
    HL,
    DE
}
