#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..8, 0..8) => Some(Self { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self {
            rank: position.rank,
            file: position.file,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank = (self.rank - other.rank).abs();
        let file = (self.file - other.file).abs();
        rank == 0 || file == 0 || rank == file
    }
}
