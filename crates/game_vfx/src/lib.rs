use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use rand::Rng;



/// Custom color palette for Bevypunk
pub trait BevypunkColorPalette {
    const BEVYPUNK_RED: Color;
    const BEVYPUNK_RED_DIM: Color;
    const BEVYPUNK_YELLOW: Color;
    const BEVYPUNK_BLUE: Color;
}
impl BevypunkColorPalette for Color {
    const BEVYPUNK_RED: Color = Color::srgba(1., 98./255., 81./255., 1.0);
    const BEVYPUNK_RED_DIM: Color = Color::srgba(172./255., 64./255., 63./255., 1.0);
    const BEVYPUNK_YELLOW: Color = Color::linear_rgba(252./255., 226./255., 8./255., 1.0);
    const BEVYPUNK_BLUE: Color = Color::srgba(8./255., 226./255., 252./255., 1.0);
}



#[derive(Component)]
pub struct VFXBloomFlicker;
impl VFXBloomFlicker {
    /// System for immitating flickering by randomly adjusting camera's bloom values
    fn system(mut query: Query<&mut Bloom, With<VFXBloomFlicker>>) {
        for mut bloom in &mut query {
            let mut rng = rand::rng();
            if rng.random_range(0..100) < 20 {
                // This formula will make the value jumping smooth and natural, like neon flicker
                bloom.intensity += (rng.random_range(0.20..0.30)-bloom.intensity)/6.0;
                bloom.prefilter.threshold += (rng.random_range(0.20..0.30)-bloom.prefilter.threshold)/4.0;
            }
        }
    }
}

/// Plugin with VFX systems for our menu
pub struct VFXPlugin;
impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, VFXBloomFlicker::system);
    }
}