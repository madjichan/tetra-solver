use std::{
    io::BufRead,
    error::Error
};

use crate::error::TetraError;
use crate::field::Field;
use crate::solution::{ Solution, SolutionDisplay };

pub struct Task {
    size: (usize, usize),
    figs: Vec<Field>
}

impl Task {
    fn next_pos(s: &mut Solution, main: &mut Field, figs: &Vec<Field>) -> bool {
        loop {
            let res = s.next(main, figs);
            if res {
                break;
            }

            if s.get_current() == 0 {
                return false;
            } else {
                s.undo(main, figs);
            }
        }

        true
    }

    fn next(s: &mut Solution, main: &mut Field, figs: &Vec<Field>) -> bool {
        if s.get_current() == figs.len() {
            s.dec();
            let res = Task::next_pos(s, main, figs);
            if !res {
                return false;
            }
        }

        loop {
            let res = s.diff(main, figs);
            if res {
                s.apply(main, figs);
                if s.get_current() == figs.len() {
                    break;
                }
            }
            else {
                let res = Task::next_pos(s, main, figs);
                if !res {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&self) -> Vec<Solution> {
        let mut res = Vec::new();
        let mut main = Field::new(self.size.0, self.size.1);
        let mut sol = Solution::new(self.figs.len());

        while Task::next(&mut sol, &mut main, &self.figs) {
            res.push(sol.snapshot());
        }

        res
    }

    pub fn parse_input(mut reader: impl BufRead) -> Result<Task, Box<dyn Error>> {
        'error: loop {

        let mut inp = String::new();

        reader.read_line(&mut inp)?;
        let size_arr: Vec<usize> = inp.split(' ').map(|x| x.trim().parse().unwrap()).collect();
        if size_arr.len() != 2 {
            break 'error;
        }

        reader.read_line(&mut inp)?;
        inp.clear();
        reader.read_line(&mut inp)?;
        let fig_count: usize = inp.trim().parse()?;
        let mut figs = Vec::new();

        for _i in 0..fig_count {
            reader.read_line(&mut inp)?;
            inp.clear();

            reader.read_line(&mut inp)?;
            let fig_size: Vec<usize> = inp.split(' ').map(|x| x.trim().parse().unwrap()).collect();
            if fig_size.len() != 2 {
                break 'error;
            }

            let (row_count, column_count) = (fig_size[0], fig_size[1]);
            let mut n_fig = Field::new(row_count, column_count);

            for ri in 0..row_count {
                inp.clear();
                reader.read_line(&mut inp)?;

                let line = inp.trim();

                if line.len() != column_count {
                    break 'error;
                }

                let mut it = line.chars();

                for ci in 0..column_count {
                    let c = if let Some(c) = it.next() {
                        c
                    } else {
                        break 'error;
                    };

                    match c {
                        '.' => {
                            n_fig.down(ri, ci);
                        },
                        '#' => {
                            n_fig.up(ri, ci);
                        },
                        _ => {
                            break 'error;
                        }
                    }
                }
            }

            figs.push(n_fig);
        }

        return Ok(Task {
            size: (size_arr[0], size_arr[1]),
            figs 
        });

        }
        
        Err(Box::new(TetraError::INPUT_FILE_SYNTAX))
    }

    pub fn display<'a>(&'a self, sol: &'a Solution) -> SolutionDisplay<'a> {
        SolutionDisplay::new(self.size, sol, &self.figs)
    }
}
