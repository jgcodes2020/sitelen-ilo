use crate::ast::{condition::Condition, object::Object};

/// A target for a command statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Target {
    Ilo,
}

/// A keyword that begins an argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArgKey {
    E,
    Tawa,
    Tan,
    Kepeken,
}

/// A keyword that denotes a block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BlockType {
    /// A run-once block.
    Pali,
    /// A looping block.
    Sike,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Comment {
    content: String,
}

/// A command statement (one which uses *o*, generally performing some action.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Command {
    pub(crate) condition: Option<Condition>,
    pub(crate) target: Option<Target>,
    action: String,
    args: Vec<(ArgKey, Object)>,
}

/// The beginning of a block, potentially with a condition or chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlockStart {
    pub(crate) chained: bool,
    pub(crate) condition: Option<Condition>,
    pub(crate) block_type: BlockType,
}

/// A condition evaluation (using *ken la*).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConditionEval {
    pub(crate) condition: Condition,
}