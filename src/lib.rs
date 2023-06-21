#![allow(clippy::type_complexity)]
mod ai;
mod buildings;
mod camera;
mod graphics;
mod grid;
mod needs;
mod pathfinding;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::ai::*;
    pub use crate::buildings::*;
    pub use crate::camera::*;
    pub use crate::graphics::*;
    pub use crate::grid::*;
    pub use crate::needs::*;
    pub use crate::pathfinding::*;
}
