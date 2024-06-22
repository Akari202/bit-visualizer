use crate::operations::{Expression, Operation, Term};
use crate::operations::rhs::RHS;

fn format_value<T>(value: &T) -> String
    where
        T: std::fmt::LowerHex +
        std::fmt::Binary +
        std::fmt::Display +
        Copy +
        Clone
{
    let bits = std::mem::size_of::<T>();
    let dec_width = bits * 3 + 2;
    let hex_width = bits * 2 + 2;
    let bin_width = bits * 8 + 2;
    // let total = bits * 13 + 14;
    format!(
        "[{:>dec_width$} | {:>hex_width$} | {:>bin_width$}]",
        value,
        format!("{:#x}", value),
        format!("{:#b}", value)
    )
}

impl<T, U> std::fmt::Display for Term<T, U>
    where
        T: std::fmt::Display +
        std::fmt::Binary +
        std::fmt::LowerHex +
        Copy +
        Clone,
        U: std::fmt::Display + RHS<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);
        match self.operation {
            Operation::None => {
                let width = width * 3 + 7;
                write!(
                    f,
                    "{:>width$}",
                    format_value(&self.initial_value)
                )
            },
            Operation::BitwiseNot |
            Operation::LogicalNot |
            Operation::LogicalNotNot => {
                let width = width + 4;
                write!(
                    f,
                    "{:>width$}{} = {}",
                    self.operation,
                    format_value(&self.initial_value),
                    format_value(&self.result)
                )
            },
            _ => {
                write!(
                    f,
                    "{} {:>width$} = {}",
                    format_value(&self.initial_value),
                    self.operation,
                    format_value(&self.result)
                )
            }
        }
    }
}

impl<T> std::fmt::Display for Operation<T>
    where
        T: std::fmt::Display +
        std::fmt::Binary +
        std::fmt::LowerHex +
        Copy +
        Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);
        match self {
            Operation::None => write!(f, ""),
            Operation::BitwiseNot => write!(f, "{:>width$}", "~"),
            Operation::BitwiseAnd(value) => write!(f, " & {}", format_value(value)),
            Operation::BitwiseOr(value) => write!(f, " | {}", format_value(value)),
            Operation::BitwiseXor(value) => write!(f, " ^ {}", format_value(value)),
            Operation::BitwiseShiftLeft(value) => write!(f, "<< {:>width$}", value),
            Operation::BitwiseShiftRight(value) => write!(f, ">> {:>width$}", value),
            Operation::LogicalNot => write!(f, "{:>width$}", "!"),
            Operation::LogicalNotNot => write!(f, "{:>width$}", "!!"),
            Operation::ArithmeticAdd(value) => write!(f, " + {}", format_value(value))
        }
    }
}

impl<T, U> std::fmt::Display for Expression<T, U>
    where
        T: std::fmt::Display +
        std::fmt::Binary +
        std::fmt::LowerHex +
        Copy +
        Clone,
        U: std::fmt::Display + RHS<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = std::mem::size_of::<T>() * 13 + 14;
        for (i, term) in self.terms.iter().enumerate() {
            writeln!(f, "{:>2}) {:>width$}", i, term)?;
        }
        Ok(())
    }
}
