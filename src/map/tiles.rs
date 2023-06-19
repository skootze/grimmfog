#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Wall,
    Floor,
    Impassable,
}

impl TileType {
    pub fn get_index(&self) -> usize {
        match self {
            TileType::Wall => 49 * 17 + 10,
            TileType::Floor => 0,
            TileType::Impassable => 49 * 13 + 6,
        }
    }
}
