use crate::data::load;
use regex::Regex;
use std::{collections::HashMap, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error: {}.", .0)]
    ParseInputError(String),
    #[error("Integer parsing error: {:?}.", .0)]
    ParseIntError(#[from] ParseIntError),
    #[error("Not yet implemented.")]
    NotImplemented,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl TryFrom<&str> for Part {
    type Error = PuzzleErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)").unwrap();
        let Some(caps) = re.captures(value) else {
            return Err(PuzzleErr::ParseInputError(value.to_string()));
        };
        Ok(Self {
            x: caps["x"].parse()?,
            m: caps["m"].parse()?,
            a: caps["a"].parse()?,
            s: caps["s"].parse()?,
        })
    }
}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.a + self.m + self.s
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleResult {
    A,
    R,
    W(String),
}

impl From<&str> for RuleResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::A,
            "R" => Self::R,
            x => Self::W(x.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CompOp {
    Greater,
    Less,
}

impl TryFrom<&str> for CompOp {
    type Error = PuzzleErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "<" => Ok(CompOp::Less),
            ">" => Ok(CompOp::Greater),
            _ => Err(PuzzleErr::ParseInputError(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PartComponent {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for PartComponent {
    type Error = PuzzleErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(PartComponent::X),
            "m" => Ok(PartComponent::M),
            "a" => Ok(PartComponent::A),
            "s" => Ok(PartComponent::S),
            _ => Err(PuzzleErr::ParseInputError(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ComparisonOperation {
    var_name: PartComponent,
    op: CompOp,
    value: u32,
}

impl ComparisonOperation {
    fn execute(&self, part: &Part) -> bool {
        let part_val = match self.var_name {
            PartComponent::X => part.x,
            PartComponent::M => part.m,
            PartComponent::A => part.a,
            PartComponent::S => part.s,
        };
        match self.op {
            CompOp::Greater => part_val > self.value,
            CompOp::Less => part_val < self.value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Comparison {
        op: ComparisonOperation,
        res: RuleResult,
    },
    Simple(RuleResult),
}

impl Rule {
    fn execute(&self, part: &Part) -> Option<RuleResult> {
        match self {
            Rule::Simple(res) => Some(res.clone()),
            Rule::Comparison { op, res } => {
                if op.execute(part) {
                    Some(res.clone())
                } else {
                    None
                }
            }
        }
    }
}

impl TryFrom<&str> for Rule {
    type Error = PuzzleErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !(value.contains('<') | value.contains('>')) {
            let res = RuleResult::from(value);
            return Ok(Rule::Simple(res));
        }
        // a<2006:qkq
        let re =
            Regex::new(r"(?<var_name>\w)(?<op_name>[<|>])(?<value>\d+):(?<res_name>\w+)").unwrap();
        let Some(caps) = re.captures(value) else {
            return Err(PuzzleErr::ParseInputError(value.to_string()));
        };
        let op = ComparisonOperation {
            var_name: PartComponent::try_from(&caps["var_name"])?,
            op: CompOp::try_from(&caps["op_name"])?,
            value: caps["value"].parse()?,
        };
        let res = RuleResult::from(&caps["res_name"]);
        Ok(Rule::Comparison { op, res })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl TryFrom<&str> for Workflow {
    type Error = PuzzleErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^(?<name>\w+)\{(?<rules>.+)\}").unwrap();
        let Some(caps) = re.captures(value) else {
            return Err(PuzzleErr::ParseInputError(value.to_string()));
        };
        let rules = &caps["rules"]
            .split(',')
            .map(Rule::try_from)
            .collect::<Result<Vec<Rule>, PuzzleErr>>()?;
        Ok(Self {
            name: caps["name"].to_string(),
            rules: rules.clone(),
        })
    }
}

impl Workflow {
    fn execute(&self, part: &Part) -> RuleResult {
        for rule in self.rules.iter() {
            if let Some(res) = rule.execute(part) {
                return res;
            }
        }
        unreachable!();
    }
}

fn parse_workflows(input: &str) -> Result<Vec<Workflow>, PuzzleErr> {
    input
        .trim()
        .lines()
        .map(Workflow::try_from)
        .collect::<Result<Vec<Workflow>, PuzzleErr>>()
}

fn parse_parts(input: &str) -> Result<Vec<Part>, PuzzleErr> {
    input
        .trim()
        .lines()
        .map(Part::try_from)
        .collect::<Result<Vec<Part>, PuzzleErr>>()
}

fn parse_input(input: &str) -> Result<(Vec<Workflow>, Vec<Part>), PuzzleErr> {
    let split_input = input.trim().split("\n\n").collect::<Vec<_>>();
    let workflows = parse_workflows(split_input[0])?;
    let parts = parse_parts(split_input[1])?;
    Ok((workflows, parts))
}

fn organize_part(part: &Part, workflows: &HashMap<&str, Workflow>) -> RuleResult {
    let mut workflow_name = "in".to_string();
    loop {
        let w: &Workflow = workflows.get(workflow_name.as_str()).unwrap();
        let res = w.execute(part);
        workflow_name = match res {
            RuleResult::A | RuleResult::R => return res,
            RuleResult::W(next_w) => next_w,
        };
    }
}

fn organize_parts(parts: &[Part], workflows: &HashMap<&str, Workflow>) -> u32 {
    parts
        .iter()
        .filter_map(|p| {
            if organize_part(p, workflows) == RuleResult::A {
                Some(p.sum())
            } else {
                None
            }
        })
        .sum()
}

pub fn puzzle_1(input: &str) -> Result<u32, PuzzleErr> {
    let (workflows, parts) = parse_input(input)?;
    let workflows_map = workflows
        .iter()
        .map(|w| (w.name.as_str(), w.clone()))
        .collect::<HashMap<&str, Workflow>>();
    Ok(organize_parts(&parts, &workflows_map))
}

pub fn puzzle_2(input: &str) -> Result<u32, PuzzleErr> {
    let (workflows, parts) = parse_input(input)?;
    let workflows_map = workflows
        .iter()
        .map(|w| (w.name.as_str(), w.clone()))
        .collect::<HashMap<&str, Workflow>>();
    Ok(organize_parts(&parts, &workflows_map))
}

pub fn main(data_dir: &str) {
    println!("Day 19: Aplenty");
    let data = load(data_dir, 19, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(509597));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    // assert_eq!(answer_2, Ok(30449))
}
