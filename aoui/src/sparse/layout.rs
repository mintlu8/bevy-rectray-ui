use bevy::prelude::*;

/// Direction on a rectangular grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AxisDir {
    Left, Right, Up, Down
}

impl AxisDir {
    pub fn vec(self) -> Vec2 {
        match self {
            AxisDir::Left => Vec2::new(-1.0, 0.0),
            AxisDir::Right => Vec2::new(1.0, 0.0),
            AxisDir::Up => Vec2::new(0.0, 1.0),
            AxisDir::Down => Vec2::new(0.0, -1.0),
        }
    }
}

/// Direction on an isometric grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum IsometricDir {
    UpLeft, UpRight,
    DownLeft, DownRight,
}

impl IsometricDir {
    pub fn vec(self) -> Vec2 {
        match self {
            IsometricDir::UpLeft => Vec2::new(-0.5, 0.5),
            IsometricDir::UpRight => Vec2::new(0.5, 0.5),
            IsometricDir::DownLeft => Vec2::new(-0.5, -0.5),
            IsometricDir::DownRight => Vec2::new(0.5, -0.5),
        }
    }
}

/// Direction on a hexagonal grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HexDir {
    Up, Down,
    UpLeft, UpRight,
    DownLeft, DownRight,
}

pub const COS30: f32 = 0.8660254;

impl HexDir {
    pub fn zigzag(&self, y: HexDir) -> Vec2 {
        let y_vec = y.flat();
        self.flat().project_onto(Vec2::new(y_vec.y, -y_vec.x))
    }

    pub fn flat(&self) -> Vec2 {
        match self {
            HexDir::Up => Vec2::new(0.0, 1.0),
            HexDir::Down => Vec2::new(0.0, -1.0),
            HexDir::UpLeft => Vec2::new(-COS30, 0.5),
            HexDir::UpRight => Vec2::new(COS30, 0.5),
            HexDir::DownLeft => Vec2::new(-COS30, -0.5),
            HexDir::DownRight => Vec2::new(COS30, -0.5),
        }
    }
}

/// A scene accepting children with a 2D position in child space.
#[derive(Debug, Clone, Reflect)]
#[non_exhaustive]
pub enum SparseLayout{
    Rectangles {
        /// The +x axis in local space.
        x: AxisDir,
        /// The +y axis in local space.
        y: AxisDir,
        /// Size of an individual cell
        size: Vec2,
    },
    Isometric {
        /// The +x axis in local space.
        x: IsometricDir,
        /// The +y axis in local space.
        y: IsometricDir,
        /// Size of an individual cell diagonally
        size: Vec2,
    },
    HexGrid {
        /// The "zig-zag" +x axis in local space.
        ///
        /// This is guaranteed to be the direction of
        /// `origin => origin + (1, 0)`
        x: HexDir,
        /// The strait +y axis in local space.
        y: HexDir,
        /// Size of an individual cell
        ///
        /// for a regular hexagon, `x = a`, `y = √3a`
        size: Vec2,
    }
}

impl Default for SparseLayout {
    fn default() -> Self {
        SparseLayout::Rectangles {
            x: AxisDir::Right,
            y: AxisDir::Up,
            size: Vec2::ONE,
        }
    }
}

impl SparseLayout {
    /// This provides a best guess for the size of the cell.
    ///
    /// Providing a custom one is preferred, espacially in rotated versions.
    pub fn cell_size_hint(&self) -> Vec2{
        match self {
            SparseLayout::Rectangles { size, .. } => *size,
            SparseLayout::Isometric { size, .. } => *size,
            SparseLayout::HexGrid { size, .. } => *size,
        }
    }

}
