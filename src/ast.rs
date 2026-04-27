use crate::scanner::Token;
use crate::scanner::LiteralValue;

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: LiteralValue,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct VarExpr {
    pub name: Token,
}

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Var(VarExpr),
}

pub trait ExprVisitor<T> {
    fn visit_binary(&self, binary: &BinaryExpr) -> T;
    fn visit_grouping(&self, grouping: &GroupingExpr) -> T;
    fn visit_literal(&self, literal: &LiteralExpr) -> T;
    fn visit_unary(&self, unary: &UnaryExpr) -> T;
    fn visit_var(&self, var: &VarExpr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(expr) => visitor.visit_binary(expr),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Literal(expr) => visitor.visit_literal(expr),
            Expr::Unary(expr) => visitor.visit_unary(expr),
            Expr::Var(expr) => visitor.visit_var(expr),
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

pub trait StmtVisitor<T> {
    fn visit_expression(&self, expression: &ExpressionStmt) -> T;
    fn visit_print(&self, print: &PrintStmt) -> T;
    fn visit_var(&self, var: &VarStmt) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Var(expr) => visitor.visit_var(expr),
        }
    }
}