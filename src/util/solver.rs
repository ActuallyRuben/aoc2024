use std::ops::{BitAnd, BitXor, Rem, Shr};

pub struct Solver {
    variables: Vec<VariableImpl>,
    constraints: Vec<Constraint>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            variables: vec![],
            constraints: vec![],
        }
    }

    pub fn unknown(&mut self) -> Variable {
        self.variables.push(VariableImpl {
            value: 0,
            unknowns: usize::MAX,
        });
        Variable(self.variables.len() - 1)
    }

    pub fn known(&mut self, value: usize) -> Variable {
        self.variables.push(VariableImpl { value, unknowns: 0 });
        Variable(self.variables.len() - 1)
    }

    pub fn constraint(&mut self, a: Variable, b: Expr) {
        self.constraints.push(Constraint(a, b))
    }

    pub fn assign(&mut self, expr: Expr) -> Variable {
        let var = self.unknown();
        self.constraint(var, expr);
        var
    }

    pub fn solve_for(&mut self, var: Variable) -> usize {
        loop {
            let var = &self.variables[var.0];
            if var.unknowns == 0 {
                return var.value;
            }
            for constraint in &self.constraints {}
        }
    }
}

struct VariableImpl {
    value: usize,
    unknowns: usize,
}

struct Constraint(Variable, Expr);

#[derive(Copy, Clone)]
pub struct Variable(usize);

pub enum Expr {
    Xor(Variable, Variable),
    Shr(Variable, Variable),
    And(Variable, Variable),
    Variable(Variable),
}

impl BitXor for Variable {
    type Output = Expr;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Expr::Xor(self, rhs)
    }
}

impl Shr for Variable {
    type Output = Expr;

    fn shr(self, rhs: Self) -> Self::Output {
        Expr::Shr(self, rhs)
    }
}

impl BitAnd for Variable {
    type Output = Expr;

    fn bitand(self, rhs: Self) -> Self::Output {
        Expr::And(self, rhs)
    }
}

impl From<Variable> for Expr {
    fn from(value: Variable) -> Self {
        Expr::Variable(value)
    }
}
