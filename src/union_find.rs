use crate::puyo::*;

pub struct UnionFind {
    pub par: Field,
    pub siz: [[usize; FIELD_WIDTH]; FIELD_HEIGHT],
}

impl UnionFind {
    pub fn new() -> Self {
        let mut par: Field = [[0; FIELD_WIDTH]; FIELD_HEIGHT];
        for i in 0..FIELD_HEIGHT {
            for j in 0..FIELD_WIDTH {
                par[i][j] = (i * FIELD_WIDTH + j) as i32;
            }
        }
        UnionFind {
            par,
            siz: [[1; FIELD_WIDTH]; FIELD_HEIGHT],
        }
    }

    pub fn root(&mut self, x: usize, y: usize) -> i32 {
        if self.par[x][y] == (x * FIELD_WIDTH + y) as i32 {
            return (x * FIELD_WIDTH + y) as i32;
        }
        self.par[x][y] = self.root(
            self.par[x][y] as usize / FIELD_WIDTH,
            self.par[x][y] as usize % FIELD_WIDTH,
        );
        self.par[x][y]
    }

    pub fn unite(&mut self, parent: &mut Position, child: &mut Position) -> bool {
        let mut parent2 = self.root(parent.x, parent.y);
        parent.x = parent2 as usize / FIELD_WIDTH;
        parent.y = parent2 as usize % FIELD_WIDTH;
        let mut child2 = self.root(child.x, child.y);
        child.x = child2 as usize / FIELD_WIDTH;
        child.y = child2 as usize % FIELD_WIDTH;

        if parent2 == child2 {
            return false;
        }

        if self.siz[parent.x][parent.y] < self.siz[child.x][child.y] {
            std::mem::swap(&mut parent2, &mut child2);
            std::mem::swap(parent, child);
        }

        self.par[child2 as usize / FIELD_WIDTH][child2 as usize % FIELD_WIDTH] = parent2;
        self.siz[parent2 as usize / FIELD_WIDTH][parent2 as usize % FIELD_WIDTH] +=
            self.siz[child.x][child.y];
        true
    }

    pub fn size(&mut self, pos: Position) -> usize {
        let root = self.root(pos.x, pos.y);
        self.siz[root as usize / FIELD_WIDTH][root as usize % FIELD_WIDTH]
    }
}
