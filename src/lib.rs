#![allow(clippy::type_complexity)]
mod ai;
mod animation;
mod app;
mod buildings;
mod camera;
mod graphics;
mod grid;
mod needs;
mod pathfinding;
mod player;
mod utils;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::ai::*;
    pub use crate::animation::*;
    pub use crate::app::*;
    pub use crate::buildings::*;
    pub use crate::camera::*;
    pub use crate::graphics::*;
    pub use crate::grid::*;
    pub use crate::needs::*;
    pub use crate::pathfinding::*;
    pub use crate::player::*;
    pub use crate::utils::*;
}
