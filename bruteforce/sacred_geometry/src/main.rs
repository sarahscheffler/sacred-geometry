/* Brute-force calculate the arithmetic formula that combines the numbers to the target, if it
 * exists.  If it doesn't, return 0.
 * Sacred Geometry ( Pathfinder feat )
 * 8/27/17
 * Sarah Scheffler
 */

/* Approach used was found here::
 * http://www.codinghelmet.com/?path=exercises/expression-from-numbers
 */

/* Encoding: N least significant bits are "is the nth number used"?.  The bits above that are the
 * value of the expression.  E.g. if the numbers are [1, 1, 4], then "1+1" is encoded as "2||110",
 * a.k.a. "10110".  1*4 is encoded as "4||101" a.k.a. "100101". 
 */

extern crate rand;

use std::vec::Vec;
use std::collections::{HashMap,  VecDeque};
use std::fmt;
use std::env;
use std::slice::Iter;
use rand::distributions::{IndependentSample, Range};

const PRIME_CONSTANTS: &'static [[u8; 3]; 9] = &[
    [3, 5, 7],
    [11, 13, 17],
    [19, 23, 29],
    [31, 37, 41],
    [43, 47, 53],
    [59, 61, 67],
    [71, 73, 79],
    [83, 89, 97],
    [101, 103, 107],
];

const HELP_MSG: &str = "Usage: `./sacred_geometry num_dice spell_level`";

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("{}", HELP_MSG);
        return
    }

    let num_dice: u8 = args[1].parse().unwrap_or(0);
    if num_dice < 1 || num_dice > 24 {
        println!("Number of dice must be between 1 and 24");
        return
    }

    let spell_level: usize = args[2].parse().unwrap_or(0);
    if spell_level < 1 || spell_level > 9 {
        println!("Spell level must be between 1 and 9");
        return
    }

    // get random die rolls
    let range = Range::new(1,7); // lower inclusive, upper exclusive
    let mut rng = rand::thread_rng();
    let mut dierolls = Vec::new();
    for _ in 0..num_dice {
        dierolls.push(range.ind_sample(&mut rng));
    }

    println!("Die rolls: {:?}", dierolls);

    let targets = PRIME_CONSTANTS[spell_level - 1]; // indexing

    for target in targets.into_iter() {
        let mut solver: Solver = Solver::new(dierolls.clone(), *target);
        solver.solve();
        if solver.has_solution() {
            solver.print_solution();
            return
        }
    }
    println!("No solution was found, aww");
}

#[derive(Clone)]
enum Operator { Add, Sub, SubReverse, Mul, Div, DivReverse }

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

impl Operator {
    fn iterator() -> Iter<'static, Operator> {
        static OPERATORS: [Operator; 6] = [Operator::Add, Operator::Sub, Operator::SubReverse, Operator::Mul, Operator::Div, Operator::DivReverse];
        OPERATORS.into_iter()
    }
}

// expressions are either made of sub-expressions or are values
#[derive(Clone)]
enum Expression { ExprParts(u32, Operator, u32), DieRoll(u8) }

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::ExprParts(lhs, ref op, rhs) => write!(f, "({:b}){}({:b})", lhs, op, rhs),
            Expression::DieRoll(x) => write!(f, "[{}]", x),
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::ExprParts(lhs, ref op, rhs) => write!(f, "({:b}){}({:b})", lhs, op, rhs),
            Expression::DieRoll(x) => write!(f, "[{}]", x),
        }
    }
}


struct Solver {
    count: u8,
    dierolls: Vec<u8>,
    target_decoded: u8,
    target_encoded: u32,
    built_exprs: HashMap<u32, Expression>, // encoded exprs either map to 
                                           // (LHS, Op, RHS) or to a die roll
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

impl Solver {

    /**
     * value: u32 - the value this expression equals.  For example, if the expression
     * is 5+1 = 6, then the value is 6.
     * numbers: u32 - The bit corresponding to 2^n means that the nth number was 
     * used in this expression.  Bits above 24 will be disregarded.  For example,
     * if the numbers are [5, 2, 2, 1] and 5 and 1 are used, then numbers will be
     * 0b1001 = 9.
     * Output: value||numbers.
     */
    fn encode_expr(&self, value: u32, numbers: u32) -> u32 {
        ((value as u32) << self.count) + (((1 << self.count) - 1) & numbers)
    }

    /** Extract the value from the expression encoding */
    fn expr_to_value(&self, expr: u32) -> u32 {
        expr >> self.count
    }

    /** Extract the numbers used from the expression encoding */
    fn expr_to_numbers(&self, expr: u32) -> u32 {
        ((1 << self.count) - 1) & expr
    }

    pub fn new(dierolls: Vec<u8>, target: u8) -> Solver {
        let count = dierolls.len();
        assert!(count <= 24);
        Solver { 
            count: count as u8,
            dierolls: dierolls,
            target_decoded: target,
            target_encoded: ((target as u32) << count) + ((1 << count) - 1),
            built_exprs: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    fn solve(&mut self) {
        for i in 0..(self.count as usize) {
            let encoded_num = self.encode_expr(
                (self.dierolls[i] as u32), (1 << i));
            let new_expr = Expression::DieRoll(self.dierolls[i]);
            self.built_exprs.insert(
                encoded_num, new_expr);
            self.queue.push_back(encoded_num);
        }

        
        while self.queue.len() > 0 && 
                    !self.built_exprs.contains_key(&self.target_encoded) {
            let lhs: u32 = self.queue.pop_front().unwrap();
            let lhs_numbers: u32 = self.expr_to_numbers(lhs);
            let lhs_value: u32 = self.expr_to_value(lhs);

            let rhs_possibilities = self.built_exprs.clone();

            for &rhs in rhs_possibilities.keys() {
                let rhs_numbers: u32 = self.expr_to_numbers(rhs);
                let rhs_value: u32 = self.expr_to_value(rhs);

                // Need LHS, RHS to not share any numbers
                if lhs_numbers & rhs_numbers != 0 {
                    continue;
                }

                for op in Operator::iterator() {
                    let new_value_opt: Option<u32> = match *op {
                        Operator::Add => Some(lhs_value + rhs_value),
                        Operator::Sub => if lhs_value >= rhs_value {
                            Some(lhs_value - rhs_value)
                        } else {
                            None
                        },
                        Operator::SubReverse => if rhs_value >= lhs_value {
                            Some(rhs_value - lhs_value)
                        } else {
                            None
                        },
                        Operator::Mul => Some(lhs_value as u32 * rhs_value as u32),
                        Operator::Div => if (rhs_value != 0) && 
                            (lhs_value % rhs_value == 0) {
                            Some(lhs_value / rhs_value)
                        } else {
                            None
                        },
                        Operator::DivReverse => if (lhs_value != 0) && 
                            (rhs_value % lhs_value == 0) {
                            Some(rhs_value / lhs_value)
                        } else {
                            None
                        },
                    };

                    if new_value_opt.is_none() {
                        continue;
                    }

                    let new_value:u32 = new_value_opt.unwrap();
                    let new_numbers:u32 = lhs_numbers | rhs_numbers;
                    let new_enc = self.encode_expr(new_value, new_numbers);

                    if !self.built_exprs.contains_key(&new_enc) {
                        let new_expr = Expression::ExprParts(lhs, op.clone(), rhs);
                        self.built_exprs.insert(new_enc, new_expr);
                        self.queue.push_back(new_enc);
                    }
                }

            }

        }

    }

    /** True if the target has been reached, false otherwise */
    fn has_solution(&self) -> bool {
        self.built_exprs.contains_key(&self.target_encoded)
    }


    /** Recursively print an Expression */
    pub fn print_expr(&self, expr: Expression) -> String {
        match expr {
            Expression::ExprParts(lhs, Operator::SubReverse, rhs) => 
                self.print_expr(Expression::ExprParts(rhs, Operator::Sub, lhs)),
            Expression::ExprParts(lhs, Operator::DivReverse, rhs) => 
                self.print_expr(Expression::ExprParts(rhs, Operator::Div, lhs)),
            Expression::ExprParts(lhs, op, rhs) => 
                format!("({}){}({})", self.printer(lhs), op, self.printer(rhs)),
            Expression::DieRoll(x) => 
                format!("{}", x),
        }
    }

    /** Recursively print the way to get to the given encoded value */
    fn printer(&self, encoded_value: u32) -> String {
        // assumes we have the target
        self.print_expr(self.built_exprs.get(&encoded_value).unwrap_or(
                &Expression::DieRoll(0)).clone())
    }

    fn print_solution(&self) {
            println!("{} = {}", self.target_decoded as u32, 
                     self.printer(self.target_encoded));
    }
}


