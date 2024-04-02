#[derive(Copy, Clone)]
pub enum Operation<T> where T: Copy + Clone {
    None,
    BitwiseNot,
    BitwiseAnd(T),
    BitwiseOr(T),
    BitwiseXor(T),
    BitwiseShiftLeft(T),
    BitwiseShiftRight(T),
    LogicalNot,
    LogicalNotNot,
    ArithmeticAdd(T)
}

#[derive(Copy, Clone)]
pub struct Term<T> where T: Copy + Clone {
    pub initial_value: T,
    pub operation: Operation<T>,
    pub result: T
}

#[derive(Clone)]
pub struct Expression<T> where T: Copy + Clone {
    pub terms: Vec<Term<T>>
}

impl<T> Term<T>
    where
        T: std::ops::BitAnd<Output = T> +
        std::ops::BitOr<Output = T> +
        std::ops::BitXor<Output = T> +
        std::ops::Shl<Output = T> +
        std::ops::Shr<Output = T> +
        std::ops::Not<Output = T> +
        std::ops::Add<Output = T> +
        PartialEq +
        From<u8> +
        From<bool> +
        Copy +
        Clone +
        Default +
        std::fmt::Display
{
    pub fn new(initial_value: T, operation: Operation<T>) -> Self {
        let mut term = Term {
            initial_value,
            operation,
            result: initial_value
        };
        term.operate();
        term
    }

    pub fn new_from_term(previous_term: &Term<T>, operation: Operation<T>) -> Self {
        let mut term = Term {
            initial_value: previous_term.result,
            operation,
            result: previous_term.result
        };
        term.operate();
        term
    }
    //
    // pub fn change_operation(&mut self, operation: Operation<T>) {
    //     self.operation = operation;
    //     self.operate();
    // }

    fn operate(&mut self) {
        self.result = match self.operation {
            Operation::None => self.initial_value,
            Operation::BitwiseNot => !self.initial_value,
            Operation::BitwiseAnd(value) => self.initial_value & value,
            Operation::BitwiseOr(value) => self.initial_value | value,
            Operation::BitwiseXor(value) => self.initial_value ^ value,
            Operation::BitwiseShiftLeft(value) => self.initial_value << value,
            Operation::BitwiseShiftRight(value) => self.initial_value >> value,
            Operation::LogicalNot => (self.initial_value == 0u8.into()).into(),
            Operation::LogicalNotNot => (self.initial_value != 0u8.into()).into(),
            Operation::ArithmeticAdd(value) => self.initial_value + value
        }
    }
}

impl<T> Expression<T>
    where
        T: std::ops::BitAnd<Output = T> +
        std::ops::BitOr<Output = T> +
        std::ops::BitXor<Output = T> +
        std::ops::Shl<Output = T> +
        std::ops::Shr<Output = T> +
        std::ops::Not<Output = T> +
        std::ops::Add<Output = T> +
        PartialEq +
        From<u8> +
        From<bool> +
        Copy +
        Clone +
        Default +
        std::fmt::Display
{
    pub fn new() -> Self {
        Expression { terms: Vec::new() }
    }

    pub fn set_initial_value(&mut self, initial_value: T) {
        if let Some(term) = self.terms.first_mut() {
            term.initial_value = initial_value;
            self.propogate();
        } else {
            self.terms.push(Term::new(initial_value, Operation::None));
        }
    }

    pub fn change_operation(&mut self, index: usize, operation: Operation<T>) {
        if let Some(term) = self.terms.get_mut(index) {
            term.operation = operation;
            self.propogate();
        }
    }


    fn propogate(&mut self) {
        if self.is_empty() {
            return;
        }
        let mut previous_term = self.terms[0];
        previous_term.operate();
        for term in self.terms.iter_mut().skip(1) {
            term.initial_value = previous_term.result;
            term.operate();
            previous_term = *term;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn final_value(&self) -> T {
        if let Some(term) = self.terms.last() {
            term.result
        } else {
            T::default()
        }
    }

    pub fn add_operation(&mut self, operation: Operation<T>) {
        if self.is_empty() {
            self.terms.push(Term::new(T::default(), operation));
        } else {
            self.terms.push(Term::new_from_term(&self.terms.last().unwrap(), operation));
        }
    }

    pub fn format_as_ops(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut formatted = self.terms[0].initial_value.to_string();
        for term in self.terms.iter().skip(1) {
            formatted = match term.operation {
                Operation::None => formatted,
                Operation::BitwiseNot => format!("~{}", formatted),
                Operation::BitwiseAnd(value) => format!("({} & {})", formatted, value),
                Operation::BitwiseOr(value) => format!("({} | {})", formatted, value),
                Operation::BitwiseXor(value) => format!("({} ^ {})", formatted, value),
                Operation::BitwiseShiftLeft(value) => format!("({} << {})", formatted, value),
                Operation::BitwiseShiftRight(value) => format!("({} >> {})", formatted, value),
                Operation::LogicalNot => format!("!{}", formatted),
                Operation::LogicalNotNot => format!("!!{}", formatted),
                Operation::ArithmeticAdd(value) => format!("({} + {})", formatted, value)
            }
        }
        formatted
    }
}

impl<T> Expression<T>
    where T: Copy + Default {

}

mod display {
    use crate::operations::{Expression, Operation, Term};

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

    impl<T> std::fmt::Display for Term<T>
        where
            T: std::fmt::Display +
            std::fmt::Binary +
            std::fmt::LowerHex +
            Copy +
            Clone
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

    impl<T> std::fmt::Display for Expression<T>
        where
            T: std::fmt::Display +
            std::fmt::Binary +
            std::fmt::LowerHex +
            Copy +
            Clone
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let width = std::mem::size_of::<T>() * 13 + 14;
            for (i, term) in self.terms.iter().enumerate() {
                writeln!(f, "{:>2}) {:>width$}", i, term)?;
            }
            Ok(())
        }
    }
}

// std::ops::BitAnd<Output = T>
// std::ops::BitOr<Output = T>
// std::ops::BitXor<Output = T>
// std::ops::Shl<Output = T>
// std::ops::Shr<Output = T>
// std::ops::Not<Output = T>
// std::ops::Add<Output = T>
// std::cmp::PartialEq
// std::convert::From<u8> +
// // std::convert::From<i8> +
// std::convert::From<bool> +
// std::fmt::Display + Copy + Default

mod opsimpl {

}
