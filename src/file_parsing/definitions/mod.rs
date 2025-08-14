pub mod definitions;

#[derive(Debug)]
pub struct DefState {
	name: String,
	state_type: DefinitionState
}

#[derive(Debug)]
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

impl DefState {
	pub fn new(name: String, state_type: DefinitionState) -> DefState {
		return DefState{ 
			name,
			state_type
		};
	}
}
