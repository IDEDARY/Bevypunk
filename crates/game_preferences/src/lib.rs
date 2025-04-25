use bevy::{app::PluginGroupBuilder, prelude::*, render::{settings::{PowerPreference, RenderCreation, WgpuSettings}, RenderPlugin}, window::{PresentMode, WindowMode, WindowResolution}};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::AudioPlugin;
use bevy_lunex::UiLunexPlugin;
use vleue_kinetoscope::AnimatedImagePlugin;
use clap::Parser;


/// Launch arguments for the Bevypunk game
#[derive(Parser, Debug, Clone, Copy)]
pub struct Args {
    /// Flag to skip the initial intro
    #[arg(short, long)]
    pub skip_intro: bool,

    /// If to launch with low ram expectations
    #[arg(short, long)]
    pub lowram: bool,

    /// Choose to run with weaker GPU
    #[arg(short, long)]
    pub powersaver: bool,

    /// Should start as windowed instead of fullscreen
    #[arg(short, long)]
    pub windowed: bool,
}


/// Plugin group implementing minimal default logic.
pub struct BevyPlugins(pub Args);
impl PluginGroup for BevyPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Embedd all assets into the binary
        builder = builder.add(EmbeddedAssetPlugin { mode: PluginMode::ReplaceDefault });

        // Add default plugins
        builder = builder.add_group(DefaultPlugins);

        // Set window plugin
        builder = builder.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevypunk".into(),
                mode: if self.0.windowed { WindowMode::Windowed } else { WindowMode::BorderlessFullscreen(MonitorSelection::Current) },
                present_mode: PresentMode::AutoVsync,
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        });

        // Set render plugin to pick high performance GPU
        builder = builder.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(
                WgpuSettings {
                    power_preference: if !self.0.powersaver { PowerPreference::HighPerformance } else { PowerPreference::LowPower },
                    ..default()
                }
            ),
            ..default()
        });

        // Disable the buildin pointers
        builder = builder.set(PointerInputPlugin { 
            is_mouse_enabled: false,
            is_touch_enabled: false,
        });

        // Add 3rd-party Bevy plugins
        builder = builder.add(AnimatedImagePlugin).add(AudioPlugin).add(UiLunexPlugin);

        // Return the plugin group
        builder
    }
}
