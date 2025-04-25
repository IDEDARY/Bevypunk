use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use bevy_lunex::*;
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



#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct AnimatedTextSlider {
    duration: f32,
    initial: String,
    step: String,
    len: usize,
    clock: f32,
    counter: usize,
}
impl Default for AnimatedTextSlider {
    fn default() -> Self {
        Self {
            duration: 0.2,
            initial: ">>>".to_string(),
            step: ">".to_string(),
            len: 12,
            clock: 0.0,
            counter: 0,
        }
    }
}
impl AnimatedTextSlider {
    /// Creates new instance
    pub fn new(initial: impl std::fmt::Display) -> Self {
        Self {
            initial: initial.to_string(),
            ..Default::default()
        }
    }
    /// Replace the default step string with a new one.
    pub fn step(mut self, step: impl std::fmt::Display) -> Self {
        self.step = step.to_string();
        self
    }
    /// Replace the default final string lenght with a new one.
    pub fn len(mut self, len: usize) -> Self {
        self.len = len;
        self
    }
    /// Replace the default step duration in seconds with a new one.
    pub fn duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }
    /// This system takes care of updating the AnimatedTextSlider in time.
    fn system(mut query: Query<(&mut Text2d, &mut AnimatedTextSlider)>, time: Res<Time>, mut commads: Commands,) {
        for (mut text, mut animator) in &mut query {
            if animator.clock < animator.duration { animator.clock += time.delta_secs(); continue; }
            animator.clock -= animator.duration;

            if animator.counter < animator.len {
                text.0 += &animator.step;
                animator.counter += 1;
            } else {
                text.0 = animator.initial.clone();
                animator.counter = 0;
            }
            commads.trigger(RecomputeUiLayout);
        }
    }
}

/// This component modifies attached [`Text2d`] with a modified string outputted from a time dependant function.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct TextAnimator {
    string: String,
    function: fn(t: f32, text: &str) -> String,
    counter: f32,
    duration: f32,
}
impl Default for TextAnimator {
    fn default() -> Self {
        Self {
            string: String::new(),
            function: decryption_animation,
            counter: 0.0,
            duration: 3.0,
        }
    }
}
impl TextAnimator {
    /// Creates new instance
    pub fn new(text: impl std::fmt::Display) -> Self {
        Self {
            string: text.to_string(),
            ..Default::default()
        }
    }
    /// Replace the default function with a new one. The function provided takes time as input and original string and outputs modified string.
    pub fn function(mut self, function: fn(t: f32, text: &str) -> String) -> Self {
        self.function = function;
        self
    }
    /// Replace the default duration in seconds with a new one.
    pub fn duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }
    /// This system takes care of updating the TextAnimator in time.
    fn system(mut query: Query<(&mut Text2d, &mut TextAnimator)>, time: Res<Time>, mut commads: Commands) {
        for (mut text, mut animator) in &mut query {

            // Increment the time counter
            if animator.counter < animator.duration { animator.counter += time.delta_secs(); }
            let just_done = animator.counter >= animator.duration;
            animator.counter = animator.counter.min(animator.duration);

            // Modify the text if changed
            if animator.counter != animator.duration || just_done {
                text.0 = (animator.function)(animator.counter/animator.duration, &animator.string);
                commads.trigger(RecomputeUiLayout);
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
    let mut rng = StdRng::seed_from_u64(seed + (t*60.0).round() as u64);

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

/// Creates a slide-in effect where characters come in from the sides
pub fn slide_in_animation(t: f32, text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let center = text.len() / 2;

    for (i, c) in text.chars().enumerate() {
        let distance_from_center = center.abs_diff(i);

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
    let mut rng = StdRng::seed_from_u64(seed);

    // Shuffle indices to determine reveal order
    use rand::seq::SliceRandom;
    indices.shuffle(&mut rng);

    let chars_to_reveal = (t * text.len() as f32).floor() as usize;
    let mut result = vec![' '; text.len()];

    // Reveal characters in scrambled order
    for i in indices.iter().take(chars_to_reveal.min(text.len())) {
        result[*i] = text.chars().nth(*i).unwrap();
    }

    result.into_iter().collect()
}



/// Plugin with VFX systems for our menu
pub struct VFXPlugin;
impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, VFXBloomFlicker::system)
            .add_systems(Update, TextAnimator::system)
            .add_systems(Update, AnimatedTextSlider::system);
    }
}