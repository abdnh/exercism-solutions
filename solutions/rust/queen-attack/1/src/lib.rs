#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..=7).contains(&rank) || !(0..=7).contains(&file) {
            None
        } else {
            Some(Self(rank, file))
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // Horizontal / vertical
        if self.0.0 == other.0.0 || self.0.1 == other.0.1 {
            return true;
        }
        // Diagonal top right
        let mut i = self.0.0;
        let mut j = self.0.1;
        while i >= 0 && j <= 7 {
            if other.0.0 == i && other.0.1 == j {
                return true;
            }
            i -= 1;
            j += 1;
        }

        // Diagonal bottom right
        let mut i = self.0.0;
        let mut j = self.0.1;
        while i <= 7 && j <= 7 {
            if other.0.0 == i && other.0.1 == j {
                return true;
            }
            i += 1;
            j += 1;
        }

        // Diagonal bottom left
        let mut i = self.0.0;
        let mut j = self.0.1;
        while i <= 7 && j >= 0 {
            if other.0.0 == i && other.0.1 == j {
                return true;
            }
            i += 1;
            j -= 1;
        }

        // Diagonal top left
        let mut i = self.0.0;
        let mut j = self.0.1;
        while i >= 0 && j >= 0 {
            if other.0.0 == i && other.0.1 == j {
                return true;
            }
            i -= 1;
            j -= 1;
        }

        false
    }
}
