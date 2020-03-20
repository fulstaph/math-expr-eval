use std::collections::BinaryHeap;
use std::ptr::null;

enum Operation {
    Addition,
    Multiplication
}

enum Number {
    Value(Option<i32>)
}

struct ExpressionNode {
    pub number: Number,
    pub operation: Operation,
}

fn main() {
    let sum = (2 + 2);

    //let mut  heap: BinaryHeap<Expression> = BinaryHeap::new();

    println!("{:#?}", sum);
}
