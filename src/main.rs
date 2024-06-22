use crate::operations::{Expression, Operation, Term};

mod operations;
mod shell;

fn main() {
    env_logger::init();
    // let mut expression: Expression<i16> = operations::Expression::new();
    // expression.set_initial_value(0x1234);
    // expression.add_operation(Operation::BitwiseNot);
    // expression.add_operation(Operation::BitwiseAnd(0x4321));
    // expression.add_operation(Operation::BitwiseOr(0x5678));
    // expression.add_operation(Operation::BitwiseShiftLeft(4));
    // expression.add_operation(Operation::BitwiseShiftRight(8));
    // expression.add_operation(Operation::LogicalNotNot);
    // println!("{}", expression.format_as_ops());
    // println!("{}", expression);
    // expression.set_initial_value(0x1);
    // println!("{}", expression.format_as_ops());
    // println!("{}", expression);

    let mut is_power_2: Expression<i16, U> = Expression::new();
    is_power_2.set_initial_value(5);
    is_power_2.add_operation(Operation::ArithmeticAdd(-1));
    is_power_2.add_operation(Operation::BitwiseAnd(is_power_2.final_value()));
    is_power_2.add_operation(Operation::LogicalNot);
    println!("{}", is_power_2.format_as_ops());
    println!("{}", is_power_2);
    is_power_2.set_initial_value(8);
    println!("{}", is_power_2.format_as_ops());
    println!("{}", is_power_2);
}
