use crate::board::Board;
use crate::classifier::Classifier;
use crate::heuristic_implementation::bruteforce::BruteForce;
use crate::heuristic_implementation::HeuristicImplementation;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SolutionNode {
    pub x: usize,
    pub y: usize,
    pub score: f64,
}

impl PartialOrd for SolutionNode {
    fn partial_cmp(&self, other: &SolutionNode) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for SolutionNode {
    fn eq(&self, other: &SolutionNode) -> bool {
        self.score == other.score
    }
}

impl SolutionNode {
    pub fn new(x: usize, y: usize, score: f64) -> Self {
        SolutionNode { x, y, score }
    }
}

#[derive(Debug)]
pub struct SolverResult {
    board: Vec<u8>,
    heuristics_description: String,
    jumps: u32,
    solution: Option<Vec<u8>>,
}

impl SolverResult {
    pub fn new(board: Vec<u8>, heuristics_description: String) -> Self {
        SolverResult {
            board,
            jumps: 0,
            heuristics_description,
            solution: None,
        }
    }

    pub fn inc_jumps(&mut self) {
        self.jumps += 1;
    }

    pub fn get_jumps(&self) -> &u32 {
        &self.jumps
    }

    pub fn set_solved(&mut self, solution: Vec<u8>) {
        self.solution = Some(solution);
    }

    pub fn is_solved(&self) -> bool {
        self.solution.is_some()
    }

    pub fn get_board(&self) -> &Vec<u8> {
        &self.board
    }

    pub fn get_solution(&self) -> &Option<Vec<u8>> {
        &self.solution
    }

    pub fn get_heuristics_description(&self) -> &String {
        &self.heuristics_description
    }
}

#[derive(Debug)]
pub struct Solver<'a> {
    classifier: Classifier<'a>,
    depleted_signatures: HashSet<Vec<u8>>,
    solver_result: SolverResult,
    max_jumps: u32,
}

impl<'a> Solver<'a> {
    pub fn new() -> Self {
        let classifier = Classifier::new();
        let depleted_signatures = HashSet::new();
        let solver_result = SolverResult::new(vec![], format!("{}", classifier));

        Solver {
            classifier,
            depleted_signatures,
            solver_result,
            max_jumps: 100000,
        }
    }

    pub fn reset(&mut self, board: &Board) {
        self.depleted_signatures = HashSet::new();
        self.solver_result = SolverResult::new(
            board.get_signature().clone(),
            format!("{}", self.classifier),
        );
    }

    pub fn set_max_jumps(&mut self, max_jumps: u32) {
        self.max_jumps = max_jumps;
    }

    pub fn push_heuristic(&mut self, hi: impl HeuristicImplementation) {
        self.classifier.push_heuristic(hi);
    }

    pub fn solve(&mut self, board: &mut Board) -> Result<&SolverResult, String> {
        if self.classifier.is_empty() {
            self.push_heuristic(BruteForce::new(1.0));
        }

        self.reset(board);
        self.exec_solve(board)
    }

    fn exec_solve(&mut self, board: &mut Board) -> Result<&SolverResult, String> {
        // TODO - Implement threads and futures
        {
            if self.solver_result.get_jumps() > &self.max_jumps {
                return Ok(&self.solver_result);
            }

            // TODO - If a depleted signature contains all bits of the tested signature,
            // then the tested signature should also be considered as depleted
            // (this will never happen in single thread mode)
            if self.depleted_signatures.contains(board.get_signature()) {
                return Ok(&self.solver_result);
            }

            let cells = board.get_available_cells();
            let mut nodes = vec![];
            for c in cells {
                let (x, y, _) = c.get_xyi();
                board.toggle_cell(&x, &y)?;

                if board.is_solved() {
                    self.solver_result.inc_jumps();
                    self.solver_result.set_solved(board.get_signature().clone());
                    return Ok(&self.solver_result);
                }

                let score = self.classifier.score(board, &x, &y);
                board.toggle_cell(&x, &y)?;

                let node = SolutionNode::new(*x, *y, score);
                nodes.push(node);
            }

            nodes.sort_by(|a, b| b.partial_cmp(a).unwrap());

            for n in nodes {
                board.toggle_cell(&n.x, &n.y)?;

                if !self.depleted_signatures.contains(board.get_signature()) {
                    self.solver_result.inc_jumps();
                    self.exec_solve(board)?;

                    if board.is_solved() {
                        self.solver_result.set_solved(board.get_signature().clone());
                        return Ok(&self.solver_result);
                    }

                    // TODO - Should be async
                    board.get_equivalent_signatures()?.iter().for_each(|s| {
                        self.depleted_signatures.insert(s.clone());
                    });
                }

                board.toggle_cell(&n.x, &n.y)?;
            }
        }

        Ok(&self.solver_result)
    }
}
