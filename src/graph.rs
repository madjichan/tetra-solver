use std::collections::HashSet;

#[derive(Clone)]
pub struct Graph {
    neighbours: Vec<HashSet<usize>>
}

#[derive(Clone, Copy)]
pub enum Color {
    RED,
    BLUE,
    GREEN,
    YELLOW,
    PURPLE,
    ORANGE
}

impl Color {
    #[allow(non_upper_case_globals)]
    const count: usize = 6;

    fn to_ind(&self) -> usize {
        match *self {
            Color::RED => 0,
            Color::BLUE => 1,
            Color::GREEN => 2,
            Color::YELLOW => 3,
            Color::PURPLE => 4,
            Color::ORANGE => 5,
        }
    }

    fn from_ind(ind: usize) -> Color {
        match ind {
            0 => Color::RED,
            1 => Color::BLUE,
            2 => Color::GREEN,
            3 => Color::YELLOW,
            4 => Color::PURPLE,
            5 => Color::ORANGE,
            _ => panic!("wtf"),
        }
    }
}

impl Graph {
    pub fn new(vert_count: usize) -> Graph {
        let neighbours = vec![HashSet::new(); vert_count];

        Graph {
            neighbours
        }
    }

    #[allow(dead_code)]
    pub fn is_con(&self, f: usize, s: usize) -> bool {
        self.neighbours[f].contains(&s)
    }

    pub fn set(&mut self, f: usize, s: usize) {
        self.neighbours[f].insert(s);
        self.neighbours[s].insert(f);
    }

    pub fn unset(&mut self, f: usize, s: usize) {
        self.neighbours[f].remove(&s);
        self.neighbours[s].remove(&f);
    }

    pub fn color(&self) -> Vec<Color> {
        let mut gr = self.clone();
        let mut cur = self.neighbours.len();
        let mut order: Vec<usize> = vec![0; self.neighbours.len()];
        let mut used: Vec<bool> = vec![false; self.neighbours.len()];
        let mut pool: HashSet<usize>;
        let mut next_pool: HashSet<usize>;

        pool = (0..self.neighbours.len()).filter(|&x: &usize| <HashSet<usize>>::len(&gr.neighbours[x]) <= 5).collect();

        while pool.len() > 0 {
            next_pool = HashSet::new();
            for p in &pool {
                cur -= 1;
                order[cur] = *p;
                used[*p] = true;
            }

            for p in pool {
                for n in &gr.neighbours[p] {
                    if !used[*n] {
                        next_pool.insert(*n);
                    }
                }

                let nbs: Vec<usize> = gr.neighbours[p].iter().map(|x| *x).collect();
                for n in nbs {
                    gr.unset(n, p);
                }
            }

            pool = next_pool.into_iter().filter(|&x: &usize| <HashSet<usize>>::len(&gr.neighbours[x]) <= 5).collect();
        }

        let mut res: Vec<Option<Color>> = vec![None; self.neighbours.len()];
        let mut used: Vec<bool>;

        for o in order {
            used = vec![false; Color::count];
            
            for n in &gr.neighbours[o] {
                used[res[*n].unwrap().to_ind()] = true;
            }

            let ci = used.into_iter().enumerate().find_map(|(i, x)| if x { None } else { Some(i) }).unwrap();
            res[o] = Some(Color::from_ind(ci));

            for n in &self.neighbours[o] {
                gr.set(o, *n);
            }
        }

        res.into_iter().map(|x| x.unwrap()).collect()
    }
}
