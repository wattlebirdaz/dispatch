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

pub struct Add<L, R>
where
    L: ExprTrait,
    R: ExprTrait,
{
    pub left: L,
    pub right: R,
}

impl<L, R> ExprTrait for Add<L, R>
where
    L: ExprTrait,
    R: ExprTrait,
{
    fn eval(&self) -> i32 {
        self.left.eval() + self.right.eval()
    }
}

pub struct Sub<L, R>
where
    L: ExprTrait,
    R: ExprTrait,
{
    pub left: L,
    pub right: R,
}

impl<L, R> ExprTrait for Sub<L, R>
where
    L: ExprTrait,
    R: ExprTrait,
{
    fn eval(&self) -> i32 {
        self.left.eval() - self.right.eval()
    }
}

pub struct Mul<L, R>
where
    L: ExprTrait,
    R: ExprTrait,
{
    pub left: L,
    pub right: R,
}

impl<L, R> ExprTrait for Mul<L, R>
where
    L: ExprTrait,
    R: ExprTrait,
{
    fn eval(&self) -> i32 {
        self.left.eval() * self.right.eval()
    }
}

pub struct Div<L, R>
where
    L: ExprTrait,
    R: ExprTrait,
{
    pub left: L,
    pub right: R,
}

impl<L, R> ExprTrait for Div<L, R>
where
    L: ExprTrait,
    R: ExprTrait,
{
    fn eval(&self) -> i32 {
        self.left.eval() / self.right.eval()
    }
}

// Note: We cannot write a parser that returns impl ExprTrait
// because the input is not known at compile time.
