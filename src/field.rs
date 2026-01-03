type Coord = (usize, usize);
// TODO: replace (usize, usize) by Coord

#[derive(Clone, Debug)]
pub struct Field {
    cells: Vec<Vec<bool>>
}

impl Field {
    pub fn new(row_count: usize, column_count: usize) -> Field {
        let cells = vec![vec![false; column_count]; row_count];

        Field {
            cells
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cells.len(), self.cells[0].len())
    }

    fn is_in(&self, main_coord: Option<Coord>, coord: Coord) -> bool {
        let main_coord = if let Some(c) = main_coord {
            c
        } else {
            (0, 0)
        };

        let size = self.size();

        if main_coord.0 > coord.0 || main_coord.1 > coord.1 {
            return false;
        }

        let r = coord.0 - main_coord.0;
        let c = coord.1 - main_coord.1;

        r < size.0 && c < size.1
    }

    pub fn get(&self, r: usize, c: usize) -> bool {
        self.cells[r][c]
    }

    pub fn set(&mut self, r: usize, c: usize, v: bool) {
        self.cells[r][c] = v;
    }

    pub fn up(&mut self, r: usize, c: usize) {
        self.set(r, c, true);
    }

    pub fn down(&mut self, r: usize, c: usize) {
        self.set(r, c, false);
    }

    fn can_be_neighbours(figure1: &Field, figure1_coord: (usize, usize), figure2: &Field, figure2_coord: (usize, usize)) -> bool {
        let figure1_size = figure1.size();
        let figure2_size = figure2.size();
        figure1_coord.0 + figure1_size.0 >= figure2_coord.0 && figure1_coord.0 < figure2_coord.0 + figure2_size.0 + 1 ||
        figure1_coord.1 + figure1_size.1 >= figure2_coord.1 && figure1_coord.1 < figure2_coord.1 + figure2_size.1 + 1
    }

    pub fn is_neighbours(figure1: &Field, figure1_coord: (usize, usize), figure2: &Field, figure2_coord: (usize, usize)) -> Option<bool> {
        if !Field::diff(figure1, Some(figure1_coord), figure2, figure2_coord) {
            return None;
        }

        if !Field::can_be_neighbours(figure1, figure1_coord, figure2, figure2_coord) {
            return Some(false);
        }

        const DELTAS: [(usize, usize); 8] = [(0, 1), (0, 2), (1, 2), (2, 2), (2, 1), (2, 0), (1, 0), (0, 0)];
        for (dr, dc) in DELTAS {
            if !Field::diff(figure1, Some((figure1_coord.0 + 1, figure1_coord.1 + 1)), figure2, (figure2_coord.0 + dr, figure2_coord.1 + dc)) {
                return Some(true)
            }
        }

        Some(false)
    }

    pub fn diff(main: &Field, main_coord: Option<(usize, usize)>, figure: &Field, figure_coord: (usize, usize)) -> bool {
        let main_coord = if let Some(c) = main_coord {
            c
        } else {
            (0, 0)
        };

        let figure_size = figure.size();

        for fr in 0..figure_size.0 {
        for fc in 0..figure_size.1 {
            
            let figure_cell = figure.get(fr, fc);
            
            let main_cell = if main.is_in(Some(main_coord), (figure_coord.0 + fr, figure_coord.1 + fc)) {
                main.get(figure_coord.0 + fr - main_coord.0, figure_coord.1 + fc - main_coord.1)
            } else {
                false
            };

            if figure_cell && main_cell {
                return false;
            }
        }}

        true
    }

    pub fn apply(main: &mut Field, fig_diff: &Field, figure_coord: (usize, usize)) {
        let figdiff_size = fig_diff.size();

        for fr in 0..figdiff_size.0 {
        for fc in 0..figdiff_size.1 {
            let (mr, mc) = (fr + figure_coord.0, fc + figure_coord.1);
            
            let figdiff_cell = fig_diff.get(fr, fc);

            if figdiff_cell {
                main.up(mr, mc);
            }
        }}
    }

    pub fn undo(main: &mut Field, fig_diff: &Field, figure_coord: (usize, usize)) {
        let figdiff_size = fig_diff.size();

        for fr in 0..figdiff_size.0 {
        for fc in 0..figdiff_size.1 {
            let (mr, mc) = (fr + figure_coord.0, fc + figure_coord.1);
            
            let figdiff_cell = fig_diff.get(fr, fc);

            if figdiff_cell {
                main.down(mr, mc);
            }
        }}
    }
    
    pub fn next(main: &Field, figure: &Field, figure_coord: (usize, usize)) -> Option<(usize, usize)> {
        let figure_size = figure.size();
        let main_size = main.size();

        if figure_coord.0 + figure_size.0 == main_size.0 && figure_coord.1 + figure_size.1 == main_size.1 {
            None
        }
        else if figure_coord.1 + figure_size.1 == main_size.1 {
            Some((figure_coord.0 + 1, 0))
        }
        else {
            Some((figure_coord.0, figure_coord.1 + 1))
        }
    }
}
