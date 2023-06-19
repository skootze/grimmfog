#[derive(Debug)]
struct Digger {
    stack: Vec<Position>,
    current_position: Position,
    tiles_visited: usize,
}

impl Digger {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..Map::WIDTH - 1);
        let y = rng.gen_range(1..Map::HEIGHT - 1);

        Self {
            stack: Vec::new(),
            current_position: Position::new(x, y),
            tiles_visited: 0,
        }
    }

    fn walk(&mut self, tiles: &mut Vec<Vec<TileType>>) {
        let curr_x = self.current_position.x;
        let curr_y = self.current_position.y;

        // Mark the current position as a Floor
        tiles[curr_y][curr_x] = TileType::Floor;

        // Get a list of possible directions
        let mut directions = [(0, -2), (0, 2), (-2, 0), (2, 0)];
        directions.shuffle(&mut rand::thread_rng());

        let mut found = false;

        for &(dir_x, dir_y) in &directions {
            // Next position
            let new_x = curr_x as i32 + dir_x;
            let new_y = curr_y as i32 + dir_y;

            // Check if the next position is within the maze and not already visited
            if new_x >= 0
                && new_x < tiles[0].len() as i32
                && new_y >= 0
                && new_y < tiles.len() as i32
                && matches!(tiles[new_y as usize][new_x as usize], TileType::Wall)
            {
                // Change the wall between the current and next cell to a Floor
                tiles[(curr_y as i32 + dir_y / 2) as usize][(curr_x as i32 + dir_x / 2) as usize] =
                    TileType::Floor;

                // Push current position to stack and update current position
                self.stack.push(self.current_position);
                self.current_position = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                };

                // Recurse
                self.walk(tiles);

                found = true;
                break;
            }
        }

        if !found {
            if let Some(pos) = self.stack.pop() {
                self.current_position = pos;
                self.walk(tiles);
            }
        }
    }
}
