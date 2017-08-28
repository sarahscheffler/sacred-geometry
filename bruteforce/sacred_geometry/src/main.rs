/* Brute-force calculate the arithmetic formula that combines the numbers to the target, if it
 * exists.  If it doesn't, return 0.
 * Sacred Geometry ( Pathfinder feat )
 * 8/27/17
 * Sarah Scheffler
 */

/* Approach used was found here::
 * http://www.codinghelmet.com/?path=exercises/expression-from-numbers
 */

/*
 * E = set of all valid expressions
 * Q = queue of expressions that have not been expanded yet
 * N = set of input numbers
 * v = target value
 *
 * Goal: Create an expression N->v using all numbers in N to produce value v
 *
 * for all numbers n in N:
 *    Add expression n->n to Q
 *    Add expression n->n to E
 *
 * while Q is not empty and E does not contain N->v:
 *     e = expression dequeued from Q
 *     for each expression f in E:
 *         G = set of expressions obtained by combining e and f
 *         for each expression g in g:
 *             if g is not in E:
 *                 Enqueue g to Q
 *                 Add g to E
 * 
 * if E contains N->v:
 *     print N->v
 */

/* Encoding: N least significant bits are "is the nth number used"?.  The bits above that are the
 * value of the expression.  E.g. if the numbers are [1, 1, 4], then "1+1" is encoded as "2||110",
 * a.k.a. "10110".  1*4 is encoded as "4||101" a.k.a. "100101". 
 */

use std::vec::Vec;
use std::collections::{HashMap,  VecDeque};
use std::fmt;

fn main() {
    // TODO: should be user input
    let mut dierolls = vec![2, 3, 5, 5];
    // Highest sacred prime is 107
    let target: u8 = 7; // (5 / 5) + (2 * 3)
    let mut solver: Solver = Solver::new(dierolls, target);
    solver.solve();
}

enum Operator { Add, Sub, SubReverse, Mul, Div, DivReverse }

// expressions are either made of sub-expressions or are values
enum Expression { ExprParts(u32, Operator, u32), DieRoll(u8) }

struct Solver {
    count: u8,
    dierolls: Vec<u8>,
    target_decoded: u8,
    target_encoded: u32,
    built_exprs: HashMap<u32, Expression>, // encoded exprs either map to (LHS, Op, RHS) or to a die roll
    queue: VecDeque<u32>, // queue of remaining encoded exprs
}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[(t:{}, c:{})({:b}){:?}]\nbuilt:{:?}\nqueue:{:?}", 
               self.target_decoded, self.count, 
               self.target_encoded, self.dierolls,
               self.built_exprs, self.queue)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::ExprParts(lhs, ref op, rhs) => write!(f, "({}){}({})", lhs, op, rhs),
            Expression::DieRoll(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::ExprParts(lhs, ref op, rhs) => write!(f, "({}){}({})", lhs, op, rhs),
            Expression::DieRoll(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match *self {
            Operator::Add => " + ",
            Operator::Sub => " - ",
            Operator::SubReverse => "<->",
            Operator::Mul => " * ",
            Operator::Div => " / ",
            Operator::DivReverse => "</>",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match *self {
            Operator::Add => " + ",
            Operator::Sub => " - ",
            Operator::SubReverse => "<->",
            Operator::Mul => " * ",
            Operator::Div => " / ",
            Operator::DivReverse => "</>",
        };
        write!(f, "{}", op)
    }
}

impl Solver {

    fn solved_exprs(&self) -> std::collections::hash_map::Keys<u32, Expression> {
        self.built_exprs.keys()
    }

    /**
     * value: u8 - the value this expression equals.  For example, if the expression
     * is 5+1 = 6, then the value is 6.
     * numbers: u32 - The bit corresponding to 2^n means that the nth number was 
     * used in this expression.  Bits above 24 will be disregarded.  For example,
     * if the numbers are [5, 2, 2, 1] and 5 and 1 are used, then numbers will be
     * 0b1001 = 9.
     * Output: value||numbers.
     */
    fn encode_expr(&self, value: u8, numbers: u32) -> u32 {
        ((value as u32) << self.count) + (((1 << self.count) - 1) & numbers)
    }

    /** Same as above, but generically, with a param count */
    pub fn encode_expr_with_count(value: u8, numbers: u32, count: u8) -> u32 {
        assert!(count <= 24);
        ((value as u32) << count) + (((1 << count) - 1) & numbers)
    }

    /** Extract the value from the expression encoding */
    fn expr_to_value(&self, expr: u32) -> u8 {
        (0xff & (expr >> self.count)) as u8
    }

    /** Extract the value from the expression encoding */
    pub fn expr_to_value_with_count(expr: u32, count: u8) -> u8 {
        (0xff & (expr >> count)) as u8
    }

    /** Extract the numbers used from the expression encoding */
    fn expr_to_numbers(&self, expr: u32) -> u32 {
        ((1 << self.count) - 1) & expr
    }

    /** Extract the numbers used from the expression encoding */
    pub fn expr_to_numbers_with_count(expr: u32, count: u8) -> u32 {
        ((1 << count) - 1) & expr
    }

    pub fn new(dierolls: Vec<u8>, target: u8) -> Solver {
        let count = dierolls.len();
        assert!(count <= 24);
        Solver { 
            count: count as u8,
            dierolls: dierolls,
            target_decoded: target,
            target_encoded: Solver::encode_expr_with_count(target, ((1 << count) - 1), count as u8),
            built_exprs: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    fn solve(&mut self) {
        for i in 0..(self.count as usize) {
            let encoded_num = self.encode_expr(
                self.dierolls[i] << self.count, (1 << i));
            self.built_exprs.insert(
                encoded_num, Expression::DieRoll(self.dierolls[i]));
            self.queue.push_back(encoded_num);
        }
        println!("{:?}", self.built_exprs);
        println!("{:?}", self.queue);
    }
}


