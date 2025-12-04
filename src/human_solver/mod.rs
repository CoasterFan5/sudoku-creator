use crate::{
    grid::Grid,
    human_solver::{
        hidden_single::solve_hidden_single,
        locked_candidate_pointing::solve_locked_candidate_pointing,
        naked_single::solve_naked_single, solve_details::SolveDetails,
    },
};

mod hidden_single;
mod locked_candidate_pointing;
mod naked_single;
mod solve_details;

struct HumanSolveTechnique {
    score: i32,
    solver: fn(&mut Grid) -> Option<SolveDetails>,
    human_name: &'static str,
}

pub struct HumanSolverResponse {
    pub score: i32,
    pub techniques_used: Vec<String>,
}

/*
 * Takes in a grid and returns the score using human solvers
 */
pub fn human_solver(grid: &mut Grid) -> HumanSolverResponse {
    let mut techniques_used: Vec<String> = vec![];

    let human_solve_techiques = [
        HumanSolveTechnique {
            score: 1,
            solver: solve_naked_single,
            human_name: "Naked Single",
        },
        HumanSolveTechnique {
            score: 5,
            solver: solve_hidden_single,
            human_name: "Hidden Single",
        },
        HumanSolveTechnique {
            score: 15,
            solver: solve_locked_candidate_pointing,
            human_name: "Locked Candidate Pointing",
        },
    ];
    let mut score = 0;

    loop {
        let mut used_technique = false;
        for item in &human_solve_techiques {
            match (item.solver)(grid) {
                None => {}
                Some(value) => {
                    score += item.score;
                    let formatted = format!("{:?} {:?}", item.human_name, value.log_statement);
                    used_technique = true;
                    techniques_used.push(formatted);
                    break;
                }
            }
        }
        if used_technique {
            continue;
        }
        if !grid.is_solved() {
            println!("Failed to solve!");
        }

        break;
    }

    return HumanSolverResponse {
        score,
        techniques_used,
    };
}
