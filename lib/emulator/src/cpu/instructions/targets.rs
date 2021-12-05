
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

pub enum JumpCondition {
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
    HLDEC,
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

pub enum BitTarget {
    Seven,
}

pub enum FlagTarget {
    Z,
}

pub enum LoadType {
    Byte(LoadByteSource, LoadByteTarget),
    Word(LoadByteSource, LoadByteTarget),
    AFromIndirect(u8),
    IndirectFromA(u8),
    AFromByteAddress(u8),
    ByteAddressFromRegister(LoadByteSource, LoadByteTarget),
}

pub enum StackTarget {
    BC,
    HL,
    DE
}
