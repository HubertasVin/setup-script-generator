use std::fmt::Debug;

use strum_macros::{Display, EnumCount, EnumIter};

#[derive(PartialEq, Clone, Display, EnumIter, EnumCount)]
pub enum QuestionType {
    Options {
        title: String,
        questions: Vec<OptionType>,
    },
    ManualInputArray {
        title: String,
        questions: Vec<String>,
    },
}

impl Debug for QuestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionType::Options {
                title: _,
                questions: _,
            } => {
                write!(f, "Options")
            }
            QuestionType::ManualInputArray {
                title: _,
                questions: _,
            } => {
                write!(f, "Manual text input")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OptionType {
    pub value: String,
    pub is_checked: bool,
}

impl OptionType {
    pub fn new(value: String, is_checked: bool) -> OptionType {
        OptionType { value, is_checked }
    }
}
