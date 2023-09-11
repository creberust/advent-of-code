use std::fmt::Display;

/// Implementation for the instructions for the lights.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// Turn on the light.
    TurnOn,

    /// Turn off the light.
    TurnOff,

    /// Toggle the light.
    Toggle,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::TurnOn => write!(f, "turn on"),
            Instruction::TurnOff => write!(f, "turn off"),
            Instruction::Toggle => write!(f, "toggle"),
        }
    }
}
