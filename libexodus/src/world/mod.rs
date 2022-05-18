use crate::tiles;
use crate::tiles::{Tile, TileKind};

pub mod exampleworlds;
pub mod presets;

#[derive(Clone)]
pub struct GameWorld {
    /// Data that contains all tiles in this world
    data: Vec<Vec<Tile>>,
    playerspawn: (usize, usize),
}

impl GameWorld {
    pub fn new(width: usize, height: usize) -> Self {
        assert! {width > 0};
        assert! {height > 0};
        Self {
            data: vec![vec![tiles::AIR(); height]; width],
            playerspawn: (1, 1), // Default spawn point is (1,1)
        }
    }
    ///
    /// Set the tile at the given coordinate to the given value.
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) -> &mut Self {
        self.data[x][y] = tile;
        match &self.data[x][y].kind {
            TileKind::AIR => {}
            TileKind::SOLID => {}
            TileKind::DEADLY { from } => {
                //TODO
            }
            TileKind::SPECIAL => {
                //TODO
            }
            TileKind::PLAYERSPAWN => {
                self.playerspawn = (x, y);
            }
            TileKind::COIN => {}
        }
        self
    }
    ///
    /// Get the tile at the given location.
    /// If the location is outside of the map boundaries, return None instead.
    ///
    /// ```rust
    /// use libexodus::tiles;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,2);
    /// world.set(1,1,tiles::SPIKES());
    /// world.set(1,0,tiles::WALL());
    /// assert_eq!(&tiles::WALL,world.get(1,0).unwrap());
    /// assert!(world.get(2,0).is_none());
    /// assert!(world.get(0,-1).is_none());
    /// ```
    pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        self.data.get(x as usize)?.get(y as usize)
    }

    ///
    /// Fill the whole map with the given tile and delete everything else.
    ///
    /// ```rust
    /// use libexodus::tiles;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,2);
    /// world.set(1,1,tiles::SPIKES());
    /// world.fill(&tiles::WALL());
    /// assert_eq!(&tiles::WALL,world.get(1,1).unwrap());
    /// ```
    pub fn fill(&mut self, tile: &Tile) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.set(i, j, tile.clone());
            }
        }
        self
    }

    ///
    /// Get the width of this world.
    ///
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let world = GameWorld::new(69,1337);
    /// assert_eq!(69, world.width());
    /// ```
    pub fn width(&self) -> usize {
        self.data.len()
    }

    ///
    /// Get the height of this world.
    ///
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let world = GameWorld::new(69,1337);
    /// assert_eq!(1337, world.height());
    /// ```
    pub fn height(&self) -> usize {
        self.data[0].len()
    }

    pub fn player_spawn(&self) -> (usize, usize) {
        self.playerspawn
    }
}