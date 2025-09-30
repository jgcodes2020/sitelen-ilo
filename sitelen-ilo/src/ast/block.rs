use crate::ast::statement::{BlockStart, Command, Comment, ConditionEval};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IfBlock {
    pub(crate) start: BlockStart,
    pub(crate) contents: Vec<Statement>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IfLadder {
    pub(crate) blocks: Vec<IfBlock>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopBlock {
    pub(crate) start: BlockStart,
    pub(crate) contents: Vec<Box<Statement>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Statement {
    Comment(Comment),
    Command(Command),
    ConditionEval(ConditionEval),
    IfLadder(IfLadder),
    LoopBlock(LoopBlock),
}