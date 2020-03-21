
trait Expression {
    fn evaluate(&self) -> i32;
}

#[derive(Debug)]
struct Number {
    pub val: i32
}

impl Expression for Number {
    fn evaluate(&self) -> i32 {
        self.val
    }
}

struct Sum {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>
}

impl Expression for Sum {
    fn evaluate(&self) -> i32 {
        self.left.evaluate() + self.right.evaluate()
    }
}

struct Mul {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>
}


impl Expression for Mul {
    fn evaluate(&self) -> i32 {
        self.left.evaluate() * self.right.evaluate()
    }
}

fn main() {
    let e1 = Sum{left: Box::new(Number{val: 3}),
                right: Box::new(Sum{ left: Box::new(Number{ val: 2}),
                            right: Box::new(Number{ val: 2})
    })};
    let e2 = Sum{left: Box::new(Number{val:1}), right: Box::new(Number{val:2})};
    let e = Mul{left: Box::new(e1), right: Box::new(e2)};
    let sum = (2 + 2);
    println!("default sum: {:#?}\neval sum: {:#?}", sum, e.evaluate());
}

