use std::{
    fmt,
};

use crate::graph::{ Graph, Color };
use crate::field::Field;

pub struct Solution {
    coords: Vec<Option<(usize, usize)>>,
    current: usize
}

impl Solution {
    pub fn new(fig_count: usize) -> Solution {
        let mut coords = vec![None; fig_count];
        let current = 0;

        coords[0] = Some((0, 0));

        Solution {
            coords,
            current
        }
    }

    pub fn get_current(&self) -> usize {
        self.current
    }

    pub fn inc(&mut self) -> usize {
        self.current += 1;
        
        if self.current != self.coords.len() {
            self.coords[self.current] = Some((0, 0));
        }

        self.current
    }

    pub fn dec(&mut self) -> usize {
        if self.current != self.coords.len() {
            self.coords[self.current] = None;
        }
        self.current -= 1;
        self.current
    }

    pub fn snapshot(&self) -> Solution {
        Solution {
            coords: self.coords.clone(),
            current: self.current
        }
    }

    pub fn graph(&self, figs: &Vec<Field>) -> Graph {
        let mut res = Graph::new(self.current);

        for f in 0..self.current {
        for s in (f+1)..self.current {
            if Field::is_neighbours(&figs[f], self.coords[f].unwrap(), &figs[s], self.coords[s].unwrap()).unwrap() {
                res.set(f, s);
            }
        }
        }

        res
    }

    pub fn next(&mut self, main: &Field, figs: &Vec<Field>) -> bool {
        let opt_coord = Field::next(
            main,
            &figs[self.current],
            self.coords[self.current].unwrap()
        );

        if let Some(coord) = opt_coord {
            self.coords[self.current] = Some(coord);
            true
        } else {
            false
        }
    }

    pub fn diff(&self, main: &Field, figs: &Vec<Field>) -> bool {
        Field::diff(
            main,
            None,
            &figs[self.current],
            self.coords[self.current].unwrap()
        )
    }

    pub fn apply(&mut self, main: &mut Field, figs: &Vec<Field>) {
        Field::apply(
            main,
            &figs[self.current],
            self.coords[self.current].unwrap()
        );

        self.inc();
    }

    pub fn undo(&mut self, main: &mut Field, figs: &Vec<Field>) {
        self.dec();

        Field::undo(
            main,
            &figs[self.current],
            self.coords[self.current].unwrap()
        );
    }
}

pub struct SolutionDisplay<'a> {
    main_size: (usize, usize),
    sol: &'a Solution,
    figs: &'a Vec<Field>
}

impl<'a> SolutionDisplay<'a> {
    pub fn new(main_size: (usize, usize), sol: &'a Solution, figs: &'a Vec<Field>) -> SolutionDisplay<'a> {
        SolutionDisplay {
            main_size,
            sol,
            figs
        }
    }
}

impl<'a> fmt::Display for SolutionDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let colors = self.sol.graph(self.figs).color();

        let (mr, mc) = self.main_size;
        let mut res: Vec<Vec<char>> = vec![vec!['.'; mc]; mr];

        for (i, f) in self.figs.iter().enumerate() {
            let (fcr, fcc) = self.sol.coords[i].unwrap();
            let (fsr, fsc) = f.size();
            for r in 0..fsr {
            for c in 0..fsc {
                if f.get(r, c) {
                    res[r + fcr][c + fcc] = match colors[i] {
                        Color::RED => '@',
                        Color::BLUE => '#',
                        Color::GREEN => '$',
                        Color::YELLOW => '%',
                        Color::PURPLE => '&',
                        Color::ORANGE => '*'
                    }
                }
            }
            }
        }

        write!(f, "{}", res.into_iter().map(|x| x.into_iter().map(|x| x.to_string()).collect::<String>()).fold(String::new(), |acc, s| acc + "\n" + &s))
    }
}
