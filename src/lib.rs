#![allow(clippy::type_complexity)]
mod graphics;
mod grid;
mod pathfinding;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::graphics::*;
    pub use crate::grid::*;
    pub use crate::pathfinding::*;
}
