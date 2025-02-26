use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use rand::{Rng, SeedableRng, rngs::StdRng};



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








/// Simulates typing animation with an underscore cursor
pub fn typing_animation(t: f32, text: &str) -> String {
    let visible_chars = (t * text.len() as f32).floor() as usize;
    let visible_chars = visible_chars.min(text.len());
    
    if visible_chars < text.len() {
        // Show typed characters plus cursor
        format!("{}{}", &text[..visible_chars], "_")
    } else {
        // All characters visible, show cursor at end
        text.to_string()
    }
}

/// Creates a decryption effect where random symbols gradually become the actual text
pub fn decryption_animation(t: f32, text: &str) -> String {

    // Hash input data into unique seed
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let seed: u64 = hasher.finish();

    // Create unique reproducible RNG from time
    let mut rng = StdRng::seed_from_u64(seed + (t*20.0).round() as u64);

    // Define symbols used
    let symbols = "!@#$%^&*()_+-=[]{}|;:'\",.<>/?`~";
    let mut result = String::with_capacity(text.len());
    
    for (i, c) in text.chars().enumerate() {
        let char_progress = (t * text.len() as f32) - i as f32;
        
        if char_progress < 0.0 {
            // Not yet started decrypting this character
            result.push(symbols.chars().nth(rng.random_range(0..symbols.len())).unwrap());
        } else if char_progress >= 1.0 {
            // This character is fully decrypted
            result.push(c);
        } else {
            // This character is in the process of being decrypted
            // 80% chance of showing the real character as we get closer to 1.0
            if rng.random::<f32>() < char_progress {
                result.push(c);
            } else {
                result.push(symbols.chars().nth(rng.random_range(0..symbols.len())).unwrap());
            }
        }
    }
    
    result
}

/// Creates a fade-in effect where characters gradually appear by increasing opacity
/// In terminal this is simulated with different characters of increasing "density"
pub fn fade_in_animation(t: f32, text: &str) -> String {
    let visible_chars = (t * text.len() as f32 * 2.0).floor() as usize;
    let fully_visible = visible_chars.min(text.len());
    let partially_visible = (visible_chars.saturating_sub(text.len())).min(text.len());
    
    let density_chars = " .:;+=xX$&@#".chars().collect::<Vec<_>>();
    let max_density = density_chars.len() - 1;
    
    let mut result = String::with_capacity(text.len());
    
    for (i, c) in text.chars().enumerate() {
        if i < fully_visible {
            // Fully visible characters
            result.push(c);
        } else if i < fully_visible + partially_visible {
            // Partially visible characters (simulated with "density")
            let progress = t * 2.0 - (i as f32 / text.len() as f32);
            let density_index = (progress * max_density as f32).round() as usize;
            result.push(density_chars[density_index]);
        } else {
            // Not yet visible
            result.push(' ');
        }
    }
    
    result
}

/// Creates a slide-in effect where characters come in from the sides
pub fn slide_in_animation(t: f32, text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let center = text.len() / 2;
    
    for (i, c) in text.chars().enumerate() {
        let distance_from_center = if i < center {
            center - i
        } else {
            i - center
        };
        
        let char_progress = t * 2.0 - (distance_from_center as f32 / center as f32);
        
        if char_progress >= 1.0 {
            // Character is fully visible
            result.push(c);
        } else if char_progress > 0.0 {
            // Character is sliding in
            result.push('_');
        } else {
            // Character hasn't started appearing yet
            result.push(' ');
        }
    }
    
    result
}

/// Reveals characters in a scrambled order
pub fn scrambled_reveal_animation(t: f32, text: &str) -> String {
    // Create a seeded RNG for consistent scrambling
    let mut indices: Vec<usize> = (0..text.len()).collect();
    let seed = 42; // Fixed seed for consistent scrambling
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    
    // Shuffle indices to determine reveal order
    use rand::seq::SliceRandom;
    indices.shuffle(&mut rng);
    
    let chars_to_reveal = (t * text.len() as f32).floor() as usize;
    let mut result = vec![' '; text.len()];
    
    // Reveal characters in scrambled order
    for i in 0..chars_to_reveal.min(text.len()) {
        let idx = indices[i];
        result[idx] = text.chars().nth(idx).unwrap();
    }
    
    result.into_iter().collect()
}













/// Plugin with VFX systems for our menu
pub struct VFXPlugin;
impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, VFXBloomFlicker::system);
    }
}