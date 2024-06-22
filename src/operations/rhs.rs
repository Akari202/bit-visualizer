use crate::operations::Expression;

pub(crate) trait RHS<T> {
    fn value(&self) -> T;
}

impl<T, U> RHS<T> for Expression<T, U> where T: Copy + Clone + Default {
    fn value(&self) -> T {
        self.final_value()
    }
}

impl RHS<i8> for i8 {
    fn value(&self) -> i8 {
        *self
    }
}

impl RHS<i16> for i16 {
    fn value(&self) -> i16 {
        *self
    }
}

impl RHS<i32> for i32 {
    fn value(&self) -> i32 {
        *self
    }
}

impl RHS<i64> for i64 {
    fn value(&self) -> i64 {
        *self
    }
}

impl RHS<i128> for i128 {
    fn value(&self) -> i128 {
        *self
    }
}

impl RHS<u8> for u8 {
    fn value(&self) -> u8 {
        *self
    }
}

impl RHS<u16> for u16 {
    fn value(&self) -> u16 {
        *self
    }
}

impl RHS<u32> for u32 {
    fn value(&self) -> u32 {
        *self
    }
}

impl RHS<u64> for u64 {
    fn value(&self) -> u64 {
        *self
    }
}

impl RHS<u128> for u128 {
    fn value(&self) -> u128 {
        *self
    }
}
