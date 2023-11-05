pub mod menu;
pub use menu::Menu;


use bevy::prelude::{Plugin, App};
pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app;
    }
}