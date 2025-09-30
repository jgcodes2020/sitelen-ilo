

/// One of the primitive data types (*toki*, *nanpa*, or *lon*).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PrimitiveType {
    /// UTF-8 string.
    Toki,
    /// 64-bit signed integer.
    Nanpa,
    /// Boolean value.
    Lon,
}


/// A value that has a [`PrimitiveType`]
pub(crate) trait TypedValue {
    /// Get the type of the object.
    fn get_type(&self) -> PrimitiveType;
}

/// A literal value (of either *toki*, *nanpa*, or *lon*).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Literal {
    Toki(String),
    Nanpa(i64),
    Lon(bool),
}
impl TypedValue for Literal {
    fn get_type(&self) -> PrimitiveType {
        match self {
            Literal::Toki(_) => PrimitiveType::Toki,
            Literal::Nanpa(_) => PrimitiveType::Nanpa,
            Literal::Lon(_) => PrimitiveType::Lon,
        }
    }
}

/// A named variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Variable {
    pub(crate) ptype: PrimitiveType,
    pub(crate) name: String,
}
impl TypedValue for Variable {
    fn get_type(&self) -> PrimitiveType {
        self.ptype
    }
}

/// A reference to *ni*, the special last-result variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NiRef {
    pub(crate) ptype: PrimitiveType,
}
impl TypedValue for NiRef {
    fn get_type(&self) -> PrimitiveType {
        self.ptype
    }
}

/// An object with a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Object {
    Variable(Variable),
    Literal(Literal),
    Ni(NiRef),
}