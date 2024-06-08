#[derive(Debug, Clone)]
pub enum QuestionType {
    Options(Vec<OptionType>),
    ManualInputArray(Vec<String>),
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

impl QuestionType {
    pub fn add_option(&mut self, option: OptionType) {
        match self {
            QuestionType::Options(options) => options.push(option),
            _ => panic!("Cannot add option to non-option question type"),
        }
    }

    pub fn add_manual_input(&mut self, value: String) {
        match self {
            QuestionType::ManualInputArray(values) => values.push(value),
            _ => panic!("Cannot add manual input to non-manual input question type"),
        }
    }
}
