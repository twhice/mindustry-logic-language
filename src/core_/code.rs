use std::fmt::Debug;

type T = Box<dyn Complie>;

use super::{complier::Complie, Token};
pub struct Set {
    pub sets: Vec<(Vec<char>, Expr)>,
}
impl Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::from("{");
        for set in &self.sets {
            let mut name = String::new();
            for c in &set.0 {
                name += String::from(*c).as_str();
            }
            ret += &format!("[SET {name} = {rv:?} ]", rv = set.1)
        }
        ret += "}";
        write!(f, "{}", ret)
    }
}
impl Set {
    pub fn new(sets: Vec<(Vec<char>, Expr)>) -> Self {
        Self { sets }
    }
}

pub struct CtrlIf {
    pub if_condition: Condition,
    pub if_statement: Vec<T>,
    pub elifs: Vec<(Condition, Vec<T>)>,
    pub else_statement: Vec<T>,
}
impl Debug for CtrlIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all = String::new();
        all += &format!("{:?}", self.if_condition);
        all += &cus_to_str(&self.if_statement);

        for elif in &self.elifs {
            all += &format!("{:?}", elif.0);
            all += &cus_to_str(&elif.1);
        }
        if self.else_statement.len() != 0 {
            all += &cus_to_str(&self.else_statement);
        }
        write!(f, "{all}")
    }
}
impl CtrlIf {
    pub fn new(
        condition: Condition,
        if_statement: Vec<T>,
        elifs: Vec<(Condition, Vec<T>)>,
        else_statement: Vec<T>,
    ) -> Self {
        Self {
            if_condition: condition,
            if_statement,
            elifs,
            else_statement,
        }
    }
}

#[derive(Debug)]
pub struct CtrlDef {
    pub fn_name: Vec<char>,
    pub args: Vec<Vec<char>>,
    pub statement: Vec<T>,
}
impl CtrlDef {
    pub fn new(fn_name: Vec<char>, args: Vec<Vec<char>>, statement: Vec<T>) -> Self {
        Self {
            fn_name,
            args,
            statement,
        }
    }
}
#[derive(Debug)]
pub struct CtrlWhile {
    pub condition: Condition,
    pub statements: Vec<T>,
}
impl CtrlWhile {
    pub fn new(condition: Condition, statements: Vec<T>) -> Self {
        Self {
            condition,
            statements,
        }
    }
}

#[derive(Debug)]
pub struct CtrlSwitch {
    pub condition: Expr,
    pub cases: Vec<Vec<T>>,
}
impl CtrlSwitch {
    pub fn new(condition: Expr, cases: Vec<Vec<T>>) -> Self {
        Self { condition, cases }
    }
}

#[derive(Debug)]
pub struct CtrlReturn {
    pub return_vul: Expr,
}
impl CtrlReturn {
    pub fn new(return_vul: Expr) -> Self {
        Self { return_vul }
    }
}

#[derive(Debug)]
pub struct CtrlRepeatUntil {
    pub statements: Vec<T>,
    pub condition: Condition,
}
impl CtrlRepeatUntil {
    pub fn new(statements: Vec<T>, condition: Condition) -> Self {
        Self {
            statements,
            condition,
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    Eoe(Box<Expr>, Box<Expr>, Box<Expr>),
    Oe(Box<Expr>, Box<Expr>),
    Data(Vec<char>),
    Op(Vec<char>),
    CallFn(Vec<char>, Vec<Expr>),
}
impl Expr {
    pub fn is_right_part(&self) -> bool {
        if let Expr::Op(op) = self {
            *op == ")".chars().collect::<Vec<char>>()
        } else {
            false
        }
    }
    pub fn is_left_part(&self) -> bool {
        if let Expr::Op(op) = self {
            *op == "(".chars().collect::<Vec<char>>()
        } else {
            false
        }
    }
    pub fn is_not(&self) -> bool {
        if let Expr::Op(op) = self {
            *op == "!".chars().collect::<Vec<char>>()
        } else {
            false
        }
    }
    pub fn get_data(&self) -> Vec<char> {
        match self {
            Expr::Data(r) => r.clone(),
            _ => Vec::new(),
        }
    }
}
impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eoe(arg0, arg1, arg2) => write!(f, "({:?}{:?}{:?})", arg0, arg1, arg2),
            Self::Oe(arg0, arg1) => write!(f, "({:?}{:?})", arg0, arg1),
            Self::Data(arg0) => {
                let mut lvts = String::new();
                for c in arg0 {
                    lvts += String::from(*c).as_str();
                }
                write!(f, "{}", lvts)
            }
            Self::Op(arg0) => {
                let mut lvts = String::new();
                for c in arg0 {
                    lvts += String::from(*c).as_str();
                }
                write!(f, "{}", lvts)
            }
            Self::CallFn(arg0, arg1) => {
                let mut lvts = String::new();
                for c in arg0 {
                    lvts += String::from(*c).as_str();
                }
                let mut args = String::new();
                for arg in arg1 {
                    args += &format!("{:?} ", arg);
                }
                write!(f, "{}({})", lvts, args)
            }
        }
    }
}
impl From<&Token> for Expr {
    fn from(token: &Token) -> Self {
        match token.get_type() {
            super::TokenType::Name | super::TokenType::Num => Self::Data(token.get_text().clone()),
            super::TokenType::Symbol => Self::Op(token.get_text().clone()),
            super::TokenType::Str => Self::Data(token.get_text().clone()),
            _ => todo!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Condition {
    pub lexpr: Expr,
    pub op: Vec<char>,
    pub rexpr: Expr,
}
impl Condition {
    pub fn new(lexpr: Expr, op: Vec<char>, rexpr: Expr) -> Self {
        Self { lexpr, op, rexpr }
    }
}
fn cus_to_str(cus: &Vec<T>) -> String {
    let mut ret = String::from("{");
    for cu in cus {
        ret += &format!("{:?}", cu);
    }
    ret + "}"
}
