use crate::ast::object::Object;

#[derive(Debug)]
pub(crate) struct CondEqual {
    pub(crate) a: Object,
    pub(crate) b: Object
}

#[derive(Debug)]
pub(crate) struct CondGreater {
    pub(crate) a: Object,
    pub(crate) b: Object,
}

#[derive(Debug)]
pub(crate) struct CondLess {
    pub(crate) a: Object,
    pub(crate) b: Object,
}

#[derive(Debug)]
pub(crate) struct CondAndEqual {
    pub(crate) ps: Vec<Object>,
    pub(crate) q: Object
}

#[derive(Debug)]
pub(crate) struct CondOrEqual {
    pub(crate) ps: Vec<Object>,
    pub(crate) q: Object
}

#[derive(Debug)]
pub(crate) enum Condition {
    Equal(CondEqual),
    Greater(CondGreater),
    Less(CondLess),
    AndEqual(CondAndEqual),
    OrEqual(CondOrEqual)
}