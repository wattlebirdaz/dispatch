pub enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(l, r) => l.eval() + r.eval(),
            Expr::Sub(l, r) => l.eval() - r.eval(),
            Expr::Mul(l, r) => l.eval() * r.eval(),
            Expr::Div(l, r) => l.eval() / r.eval(),
        }
    }
}
