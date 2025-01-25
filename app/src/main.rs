use bevy::utils::HashMap;
use bevy::core_pipeline::bloom::Bloom;
pub(crate) use bevy::prelude::*;
pub(crate) use bevy_kira_audio::prelude::*;
pub(crate) use bevy_lunex::*;
pub(crate) use vleue_kinetoscope::*;

pub(crate) use game_movies::*;
pub(crate) use game_vfx::*;

use bevy_embedded_assets::*;


fn main() -> AppExit {
    // ____________________________________
    // ----- NEW APPLICATION INSTANCE -----
    let mut app = App::new();

    // Bundle all game assets into the binary
    app.add_plugins(EmbeddedAssetPlugin { mode: PluginMode::ReplaceDefault});

    // Add game plugins
    app.add_plugins((DefaultPlugins, AnimatedImagePlugin, AudioPlugin));

    // ___________________________________
    // ----- PRIORITY ASSET LOADING  -----

    let mut priority_assets = PriorityAssets::default();

    // Load the game intro
    let intro = AnimatedImageLoader::load_now_from_bytes(include_bytes!("../../assets/images/movies/intro_720p.webp"),"webp", &mut app).expect("Priority load failed");
    priority_assets.video.insert("intro".to_string(), intro);

    app.insert_resource(priority_assets);

    // _________________________________
    // ----- START THE APPLICATION -----

    app.add_systems(Startup, start_intro);
    app.add_plugins((VFXPlugin, MoviePlugin));

    app.run()
}

#[derive(Resource, Default)]
pub struct PriorityAssets {
    video: HashMap<String, Handle<AnimatedImage>>,
}

fn start_intro(mut commands: Commands, asset_server: Res<AssetServer>, priority_assets: Res<PriorityAssets>) {
    // Spawn the camera
    commands.spawn((Camera2d, Camera { hdr: true, ..default() }, Bloom::OLD_SCHOOL, VFXBloomFlicker));

    // Start the intro together with music
    commands.spawn(
        Movie::play(priority_assets.video.get("intro").unwrap().clone(), asset_server.load("audio/intro.ogg")).playback(MoviePlayback::Despawn)
    ).observe(spawn_main_menu);
}

fn spawn_main_menu(_trigger: Trigger<MovieEnded>, mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands.spawn((
        UiRoot,
    )).with_children(|ui| {

        ui.spawn((
            UiLayout::window(),
            Sprite::from_image(asset_server.load("images/ui/background.png")),
        ));
    });
}
