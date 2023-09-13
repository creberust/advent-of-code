use std::fmt::Display;

/// Implementation for binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// Logical NOT
    NOT,
    /// Logical AND
    AND,
    /// Logical OR
    OR,

    /// Logical LEFT SHIFT
    LSHIFT,
    /// Logical RIGHT SHIFT
    RSHIFT,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operator::*;

        match self {
            NOT => write!(f, "not"),
            AND => write!(f, "and"),
            OR => write!(f, "or"),
            LSHIFT => write!(f, "lshift"),
            RSHIFT => write!(f, "rshift"),
        }
    }
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "NOT" => Operator::NOT,
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "LSHIFT" => Operator::LSHIFT,
            "RSHIFT" => Operator::RSHIFT,
            _ => panic!("invalid operator value: {}", value),
        }
    }
}

/// Wire identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wire(String);

impl Wire {
    /// Create a new Wire with `id`.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().into())
    }

    /// Return the identifier of the Wire.
    pub fn id(&self) -> &str {
        &self.0
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Wire {
    fn from(value: &str) -> Self {
        Wire::new(value)
    }
}

/// Implementation for an Expression.
#[derive(Debug)]
pub enum Expr {
    /// Numerical value.
    Numeral(u16),

    /// Wire Identifier.
    Identifier(Wire),

    /// Operation between two values
    Operation(Box<Expr>, Box<Expr>, Operator),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Numeral(value) => write!(f, "{}", *value),
            Expr::Identifier(wire) => write!(f, "{}", wire.id()),
            Expr::Operation(a, b, op) => write!(f, "{} {} {}", a, op, b),
        }
    }
}
