pub enum ValueType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    OPT(Box<ValueType>),
    ARR(Box<ValueType>, Option<u32>),
    DYN
}

pub struct Value(pub u64);