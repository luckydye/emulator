
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    CCF(ArithmeticTarget),
    SCF(ArithmeticTarget),
    RRA(ArithmeticTarget),
    RLA(ArithmeticTarget),
    RRCA(ArithmeticTarget),
    RRLA(ArithmeticTarget),
    CPL(ArithmeticTarget),
    BIT(ArithmeticTarget),
    RESET(ArithmeticTarget),
    SET(ArithmeticTarget),
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
}
