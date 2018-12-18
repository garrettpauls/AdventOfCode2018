#[derive(Debug, Eq, PartialEq)]
pub enum Tile {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Debug)]
pub struct State {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl State {
    pub fn from_str(input: &str) -> Result<State, String> {
        let lines: Vec<_> = input.lines().collect();
        let mut tiles = Vec::new();

        for line in lines.iter() {
            for ch in line.chars() {
                let tile = match ch {
                    '.' => Tile::Open,
                    '|' => Tile::Trees,
                    '#' => Tile::Lumberyard,
                    _ => return Err(format!("Unsupported character: {}", ch))
                };
                tiles.push(tile);
            }
        }

        Ok(State {
            width: tiles.len() / lines.len(),
            height: lines.len(),
            tiles,
        })
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_surrounding(&self, x: usize, y: usize) -> Vec<&Tile> {
        let mut surrounding = Vec::new();

        for ox in -1..=1 as i32 {
            for oy in -1..=1 as i32 {
                if ox == 0 && oy == 0 {
                    continue;
                }

                let tx = x as i32 + ox;
                let ty = y as i32 + oy;

                if tx < 0 || ty < 0 || tx >= self.width as i32 || ty >= self.height as i32 {
                    continue;
                }

                if let Some(tile) = self.tiles.get(self.get_index(tx as usize, ty as usize)) {
                    surrounding.push(tile);
                }
            }
        }

        surrounding
    }

    pub fn simulate(&self) -> State {
        let mut tiles = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let surrounding = self.get_surrounding(x, y);
                let tile = match self.tiles[self.get_index(x, y)] {
                    Tile::Open =>
                        if count_items_of(&Tile::Trees, &surrounding) >= 3 {
                            Tile::Trees
                        } else {
                            Tile::Open
                        },
                    Tile::Trees =>
                        if count_items_of(&Tile::Lumberyard, &surrounding) >= 3 {
                            Tile::Lumberyard
                        } else {
                            Tile::Trees
                        },
                    Tile::Lumberyard => {
                        let lumber = count_items_of(&Tile::Lumberyard, &surrounding);
                        let trees = count_items_of(&Tile::Trees, &surrounding);
                        if lumber > 0 && trees > 0 {
                            Tile::Lumberyard
                        } else {
                            Tile::Open
                        }
                    }
                };

                tiles.push(tile);
            }
        }

        return State {
            width: self.width,
            height: self.height,
            tiles,
        };
    }

    pub fn count_of(&self, tile: &Tile) -> usize {
        count_items_of(tile, &self.tiles.iter().collect())
    }
}

fn count_items_of(tile: &Tile, items: &Vec<&Tile>) -> usize {
    items.iter().filter(|x| **x == tile).count()
}

use std::fmt;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, tile) in self.tiles.iter().enumerate() {
            write!(f, "{}", match tile {
                Tile::Open => '.',
                Tile::Trees => '|',
                Tile::Lumberyard => '#',
            });
            if (i + 1) % self.width == 0 {
                writeln!(f);
            }
        }

        Ok(())
    }
}