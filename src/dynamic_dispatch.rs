pub trait ExprTrait {
    fn eval(&self) -> i32;
}

pub struct Num {
    pub value: i32,
}

impl ExprTrait for Num {
    fn eval(&self) -> i32 {
        self.value
    }
}

pub struct Add {
    pub left: Box<dyn ExprTrait>,
    pub right: Box<dyn ExprTrait>,
}

impl ExprTrait for Add {
    fn eval(&self) -> i32 {
        self.left.eval() + self.right.eval()
    }
}

pub struct Sub {
    pub left: Box<dyn ExprTrait>,
    pub right: Box<dyn ExprTrait>,
}

impl ExprTrait for Sub {
    fn eval(&self) -> i32 {
        self.left.eval() - self.right.eval()
    }
}

pub struct Mul {
    pub left: Box<dyn ExprTrait>,
    pub right: Box<dyn ExprTrait>,
}

impl ExprTrait for Mul {
    fn eval(&self) -> i32 {
        self.left.eval() * self.right.eval()
    }
}

pub struct Div {
    pub left: Box<dyn ExprTrait>,
    pub right: Box<dyn ExprTrait>,
}

impl ExprTrait for Div {
    fn eval(&self) -> i32 {
        self.left.eval() / self.right.eval()
    }
}
