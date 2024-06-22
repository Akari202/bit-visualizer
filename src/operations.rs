use crate::operations::rhs::RHS;

mod display;
pub(crate) mod rhs;

#[derive(Clone)]
pub enum Operation<T> where T: Copy + Clone + std::fmt::Display {
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

#[derive(Clone)]
pub struct Term<T, U> where T: Copy + Clone, U: Copy + Clone {
    pub initial_value: T,
    pub operation: Operation<U>,
    pub result: T
}

#[derive(Clone)]
pub struct Expression<T, U> where T: Copy + Clone, U: Copy + Clone {
    pub terms: Vec<Term<T, U>>
}

impl<T, U> Term<T, U>
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
        std::fmt::Display +
        RHS<T>,
        U: Copy + Clone + RHS<T>
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
            Operation::BitwiseAnd(value) => self.initial_value & value.value(),
            Operation::BitwiseOr(value) => self.initial_value | value.value(),
            Operation::BitwiseXor(value) => self.initial_value ^ value.value(),
            Operation::BitwiseShiftLeft(value) => self.initial_value << value.value(),
            Operation::BitwiseShiftRight(value) => self.initial_value >> value.value(),
            Operation::LogicalNot => (self.initial_value == 0u8.into()).into(),
            Operation::LogicalNotNot => (self.initial_value != 0u8.into()).into(),
            Operation::ArithmeticAdd(value) => self.initial_value + value.value()
        }
    }
}

impl<T, U> Expression<T, U>
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
        std::fmt::Display +
        RHS<T>,
        U: Copy + Clone + RHS<T>
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
        let mut previous_term = self.terms[0].clone();
        previous_term.operate();
        for term in self.terms.iter_mut().skip(1) {
            term.initial_value = previous_term.result;
            term.operate();
            previous_term = term.clone();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
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

impl<T, U> Expression<T, U> where T: Copy + Clone + Default, U: Copy + Clone {
    pub fn final_value(&self) -> T {
        if let Some(term) = self.terms.last() {
            term.result
        } else {
            T::default()
        }
    }
}


