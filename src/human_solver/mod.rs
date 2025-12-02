use crate::{
    grid::Grid,
    grid_utils::{col_index_from_true_index, row_index_from_true_index},
    human_solver::{
        hidden_single::solve_hidden_single, naked_single::solve_naked_single,
        solve_details::SolveDetails,
    },
};

mod hidden_single;
mod naked_single;
mod solve_details;

struct HumanSolveTechnique {
    score: i32,
    solver: fn(&mut Grid) -> Option<SolveDetails>,
    human_name: &'static str,
}

/*
 * Takes in a grid and returns the score using human solvers
 */
pub fn human_solver(grid: &mut Grid) -> i32 {
    let mut techniques_used: Vec<String> = vec![];

    let human_solve_techiques = [
        HumanSolveTechnique {
            score: 4,
            solver: solve_naked_single,
            human_name: "Naked Single",
        },
        HumanSolveTechnique {
            score: 14,
            solver: solve_hidden_single,
            human_name: "Hidden Single",
        },
    ];
    let mut score = 0;

    loop {
        let mut placed = false;
        for item in &human_solve_techiques {
            match (item.solver)(grid) {
                None => {}
                Some(value) => {
                    score += item.score;
                    let row_index = row_index_from_true_index(value.true_index) + 1;
                    let col_index = col_index_from_true_index(value.true_index) + 1;
                    placed = true;
                    let formatted = format!(
                        "{:?} r{row_index}c{col_index}={:?}",
                        item.human_name,
                        value.value.trailing_zeros() + 1
                    );
                    techniques_used.push(formatted);
                }
            }
        }
        if !placed {
            println!("Failed to solve!");
            for str in techniques_used {
                println!("{str}");
            }
            break;
        }
    }

    return score;
}
