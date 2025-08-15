pub mod definitions;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConditionState {
    name: String,
    state_type: DefinitionState,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum DefinitionState {
    Inclusive,
    Exclusive,
}

#[derive(Debug)]
pub enum Definition {
    Bloc { content: String },
    LineWithSpace { content: String },
    Definition { name: String, value: String },
    InclusiveState { names: Vec<String> },
    ExclusiveState { names: Vec<String> },
}

impl ConditionState {
    pub fn new(name: String, state_type: DefinitionState) -> ConditionState {
        return ConditionState { name, state_type };
    }

    pub fn initial() -> ConditionState {
        return ConditionState {
            name: String::from("INITIAL"),
            state_type: DefinitionState::Inclusive,
        };
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }
}
