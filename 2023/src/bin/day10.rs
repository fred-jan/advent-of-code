use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

type Loc = (usize, usize);
type Vect = (i8, i8);

#[derive(Debug, Eq, PartialEq)]
struct Tile {
    x: usize,
    y: usize,
    kind: char,
}

impl Tile {
    pub fn loc(&self) -> Loc {
        (self.x, self.y)
    }

    pub fn connects_to(&self, other: &Tile) -> bool {
        match self.kind {
            '|' => {
                (other.y < self.y && "7F|S".contains(other.kind))
                    || (other.y > self.y && "LJ|S".contains(other.kind))
            }
            '-' => {
                (other.x < self.x && "LF-S".contains(other.kind))
                    || (other.x > self.x && "J7-S".contains(other.kind))
            }
            'L' => {
                (other.y < self.y && "7F|S".contains(other.kind))
                    || (other.x > self.x && "J7-S".contains(other.kind))
            }
            'J' => {
                (other.y < self.y && "7F|S".contains(other.kind))
                    || (other.x < self.x && "LF-S".contains(other.kind))
            }
            '7' => {
                (other.y > self.y && "LJ|S".contains(other.kind))
                    || (other.x < self.x && "LF-S".contains(other.kind))
            }
            'F' => {
                (other.y > self.y && "LJ|S".contains(other.kind))
                    || (other.x > self.x && "J7-S".contains(other.kind))
            }
            'S' => {
                (other.y > self.y && "LJ|".contains(other.kind))
                    || (other.y < self.y && "7F|".contains(other.kind))
                    || (other.x > self.x && "J7-".contains(other.kind))
                    || (other.x < self.x && "LF-".contains(other.kind))
            }
            _ => false,
        }
    }
}

type StepsMap = HashMap<Loc, usize>;

#[derive(Debug)]
struct Map {
    tiles: HashMap<Loc, Tile>,
    width: usize,
    _height: usize,
}

impl Map {
    pub fn load(contents: String) -> Self {
        Self {
            tiles: contents
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(x, char)| ((x, y), Tile { x, y, kind: char }))
                })
                .collect(),
            width: contents.find("\n").unwrap(),
            _height: contents.lines().count(),
        }
    }

    pub fn start_tile(&self) -> &Tile {
        self.tiles
            .iter()
            .find(|(_, tile)| tile.kind == 'S')
            .unwrap()
            .1
    }

    pub fn get_tiles(&self, pos: Vec<Loc>) -> Vec<&Tile> {
        pos.iter().filter_map(|pos| self.tiles.get(pos)).collect()
    }

    pub fn get_adj_tiles(&self, tile: &Tile) -> Vec<&Tile> {
        self.get_tiles(vec![
            (tile.x, tile.y.saturating_sub(1)),
            (tile.x, tile.y + 1),
            (tile.x.saturating_sub(1), tile.y),
            (tile.x + 1, tile.y),
        ])
    }

    pub fn connecting_pipes(&self, tile: &Tile) -> Vec<&Tile> {
        self.get_adj_tiles(tile)
            .into_iter()
            .filter(|adj| tile.connects_to(adj))
            .collect()
    }

    pub fn steps_map(&self) -> StepsMap {
        let mut steps: usize = 0;
        let mut tiles: Vec<&Tile> = vec![self.start_tile()];
        let mut steps_map = HashMap::new();

        loop {
            tiles.iter().for_each(|tile| {
                steps_map.insert(tile.loc(), steps);
            });
            tiles = tiles
                .iter()
                .flat_map(|tile| self.connecting_pipes(tile))
                .filter(|tile| !steps_map.contains_key(&tile.loc()))
                .collect();

            if tiles.is_empty() {
                break;
            }

            steps += 1;
        }

        steps_map
    }

    pub fn direction_map(&self) -> HashMap<Loc, Vect> {
        let mut direction_map = HashMap::new();
        let mut previous_pipe: Option<&Tile> = None;
        let mut current_pipe = self.start_tile();

        loop {
            match self
                .connecting_pipes(current_pipe)
                .into_iter()
                .find(|&connecting_pipe| {
                    !direction_map.contains_key(&connecting_pipe.loc())
                        && match previous_pipe {
                            None => true,
                            Some(pipe) => connecting_pipe != pipe,
                        }
                }) {
                None => break,
                Some(pipe) => {
                    let delta = (
                        (pipe.x as isize - current_pipe.x as isize) as i8,
                        (pipe.y as isize - current_pipe.y as isize) as i8,
                    );
                    direction_map.insert(pipe.loc(), delta);

                    // If the direction changed, update previous delta to turn it into a bend (dx+dy)
                    let current_delta = direction_map.get(&current_pipe.loc()).unwrap_or(&delta);
                    if current_delta.0 != delta.0 || current_delta.1 != delta.1 {
                        direction_map.insert(
                            current_pipe.loc(),
                            (delta.0 + current_delta.0, delta.1 + current_delta.1),
                        );
                    }

                    previous_pipe = Some(current_pipe);
                    current_pipe = pipe;
                }
            }
        }

        // todo: reverse direction if ccw
        // if !self.is_clockwise(&direction_map) {
        //     return direction_map
        //         .iter()
        //         .map(|((x, y), (dx, dy))| ((x, y), (dx * -1i8, dy * -1i8)))
        //         .collect();
        // }

        direction_map
    }

    pub fn find_right_boundary(&self, origin: &Tile, bounds: &HashMap<Loc, Vect>) -> Option<&Tile> {
        (origin.x..self.width)
            .find(|&x| bounds.contains_key(&(x, origin.y)))
            .and_then(|x| self.tiles.get(&(x, origin.y)))
    }

    pub fn is_clockwise(&self, direction_map: &HashMap<Loc, Vect>) -> bool {
        let (dx, dy) = direction_map
            .get(direction_map.keys().min().unwrap())
            .unwrap();

        dx > &0i8 || dy < &0i8
    }

    pub fn enclosed_map(&self) -> HashMap<Loc, bool> {
        let mut enclosed_map = HashMap::new();
        let direction_map = self.direction_map();
        let clockwise = self.is_clockwise(&direction_map);

        self.tiles
            .values()
            .filter(|tile| !direction_map.contains_key(&tile.loc()))
            .for_each(|tile| {
                if enclosed_map.contains_key(&tile.loc()) {
                    return;
                }

                match self.find_right_boundary(tile, &direction_map) {
                    Some(boundary) => enclosed_map.insert(
                        tile.loc(),
                        match direction_map.get(&boundary.loc()) {
                            Some((_, dy)) => {
                                if clockwise {
                                    dy > &0i8
                                } else {
                                    dy < &0i8
                                }
                            }
                            None => false,
                        },
                    ),
                    None => enclosed_map.insert(tile.loc(), false),
                };
            });

        enclosed_map
    }

    pub fn furthest_tile(&self) -> usize {
        self.steps_map().values().max().unwrap().clone()
    }

    pub fn enclosed_tiles(&self) -> usize {
        self.enclosed_map()
            .values()
            .filter(|&&enclosed| enclosed)
            .count()
    }
}

fn main() {
    // 7030
    println!(
        "Part 1: {}",
        Map::load(fs::read_to_string("inputs/day10.txt").unwrap()).furthest_tile()
    );
    // 285
    println!(
        "Part 2: {}",
        Map::load(fs::read_to_string("inputs/day10.txt").unwrap()).enclosed_tiles()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile() {
        assert_eq!(
            Tile {
                kind: '7',
                x: 91,
                y: 21
            }
            .connects_to(&Tile {
                kind: '-',
                x: 90,
                y: 20
            }),
            true
        );
    }

    #[test]
    fn test_directions_map() {
        let map = Map::load(fs::read_to_string("samples/day10c.txt").unwrap());
        let directions_map = map.direction_map();

        assert_eq!(map.is_clockwise(&directions_map), false);

        // .|L-7OF-J|.
        // .|II|O|II|.
        // .L--JOL--J.

        let tile_6_5 = directions_map.get(&(6, 5)).unwrap(); // F tile
        assert_eq!(tile_6_5.0, -1); // leftwards
        assert_eq!(tile_6_5.1, 0);

        let tile_6_6 = directions_map.get(&(6, 6)).unwrap(); // | tile
        assert_eq!(tile_6_6.0, 0);
        assert_eq!(tile_6_6.1, 1); // downwards

        let tile_6_7 = directions_map.get(&(6, 7)).unwrap(); // L tile
        assert_eq!(tile_6_7.0, 0);
        assert_eq!(tile_6_7.1, 1); // downwards

        let map = Map::load(fs::read_to_string("samples/day10e.txt").unwrap());
        let directions_map = map.direction_map();

        assert_eq!(map.is_clockwise(&directions_map), true);

        // L7
        // IL
        let tile_14_5 = directions_map.get(&(14, 5)).unwrap(); // L tile
        assert_eq!(tile_14_5.0, 0); // rightwards
        assert_eq!(tile_14_5.1, 1);

        let tile_15_5 = directions_map.get(&(15, 5)).unwrap(); // 7 tile
        assert_eq!(tile_15_5.0, 1); // rightwards
        assert_eq!(tile_15_5.1, 0);

        let tile_15_6 = directions_map.get(&(15, 6)).unwrap(); // L tile
        assert_eq!(tile_15_6.0, 0);
        assert_eq!(tile_15_6.1, 1); // downwards
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Map::load(fs::read_to_string("samples/day10a.txt").unwrap()).furthest_tile(),
            4
        );
        assert_eq!(
            Map::load(fs::read_to_string("samples/day10b.txt").unwrap()).furthest_tile(),
            8
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Map::load(fs::read_to_string("samples/day10c.txt").unwrap()).enclosed_tiles(),
            4
        );
        assert_eq!(
            Map::load(fs::read_to_string("samples/day10d.txt").unwrap()).enclosed_tiles(),
            4
        );
        assert_eq!(
            Map::load(fs::read_to_string("samples/day10e.txt").unwrap()).enclosed_tiles(),
            8
        );
        assert_eq!(
            Map::load(fs::read_to_string("samples/day10f.txt").unwrap()).enclosed_tiles(),
            10
        );
    }
}
