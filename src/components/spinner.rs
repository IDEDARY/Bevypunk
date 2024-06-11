use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#


/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Spinner {
    pub index: usize,
    pub options: Vec<String>,
}

/// Event that gets triggered if we change the spinner
#[derive(Event)]
pub struct SpinnerChange {
    pub target: Entity,
    pub value: String,
}


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// Marker struct for the sandboxed UI
#[derive(Component, Debug, Default, Clone, PartialEq)]
struct SpinnerUi;

/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
struct SpinnerControl {
    chevron_left: Entity,
    chevron_right: Entity
}

/// System that builds the component UI
fn build_component (mut commands: Commands, query: Query<(Entity, &Spinner), Added<Spinner>>, assets: Res<AssetCache>) {
    for (entity, spinner) in &query {

        // This will create a private sandboxed UiTree within the entity just for the button
        commands.entity(entity).insert(
            UiTreeBundle::<SpinnerUi>::from(UiTree::new("Spinner")),
        ).with_children(|ui| {

            // Spawn chevron left
            let chevron_left = ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Left/Chevron"),

                // Add layout
                UiLayout::window().pos(Rl((50.0, 50.0))).anchor(Anchor::Center).size((Rh(45.0), Rl(60.0))).pack::<Base>(),

                // Make it non-obsructable for hit checking (mouse detection)
                Pickable::IGNORE,

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.chevron_left.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED, ..default() },
                    ..default()
                },

                // This will set the color to red
                BaseColor::new(Color::BEVYPUNK_RED.with_a(1.0)),

                // This is required to control our hover animation
                Hover::new().receiver(true),

                // This will set hover color to yellow
                HoverColor::new(Color::BEVYPUNK_YELLOW.with_l(0.68)),
            )).id();

            // Spawn chevron right
            let chevron_right = ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Right/Chevron"),

                // Add layout
                UiLayout::window().pos(Rl((50.0, 50.0))).anchor(Anchor::Center).size((Rh(45.0), Rl(60.0))).pack::<Base>(),
                
                // Make it non-obsructable for hit checking (mouse detection)
                Pickable::IGNORE,

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.chevron_right.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED, ..default() },
                    ..default()
                },

                // This will set the color to red
                BaseColor::new(Color::BEVYPUNK_RED.with_a(1.0)),

                // This is required to control our hover animation
                Hover::new().receiver(true),

                // This will set hover color to yellow
                HoverColor::new(Color::BEVYPUNK_YELLOW.with_l(0.68)),
            )).id();

            // Spawn button text
            let text = ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Image/Text"),

                // Add layout
                UiLayout::window().pos(Rl((50., 50.))).anchor(Anchor::Center).pack::<Base>(),

                // Make it non-obsructable for hit checking (mouse detection)
                Pickable::IGNORE,

                // Add text
                UiText2dBundle {
                    text: Text::from_section(spinner.options[0].clone(),
                        TextStyle {
                            font: assets.font_medium.clone(),
                            font_size: 60.0,
                            color: Color::BEVYPUNK_RED,
                        }),
                    ..default()
                },

                // This will set the color to red
                BaseColor::new(Color::BEVYPUNK_RED.with_a(1.0)),

                // This is required to control our hover animation
                Hover::new().receiver(true),

                // This will set hover color to yellow
                HoverColor::new(Color::BEVYPUNK_YELLOW.with_l(0.68)),

                // Spinner control
                SpinnerControl { chevron_left, chevron_right }
            )).id();

            ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Image"),

                // Add layout
                UiLayout::window().size(Rl((100.0, 50.0))).pack::<Base>(),

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.button_symetric_sliced.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_a(0.15), ..default() },
                    ..default()
                },

                // Make the background scalable
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),

                // Make it non-obsructable for hit checking (mouse detection)
                PickableBundle::default(),

                // This is required to control our hover animation
                Hover::new().forward_speed(20.0).backward_speed(5.0),

                // This will pipe this hover data to the specified entities
                HoverPipe::new(vec![text]),

                // This will set the color to red
                BaseColor::new(Color::BEVYPUNK_RED.with_a(0.15)),

                // This will set hover color to yellow
                HoverColor::new(Color::BEVYPUNK_YELLOW.with_l(0.68)),
            ));

            ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Left"),

                // Add layout
                UiLayout::window().pos(Rl((0.0, 55.0))).size((Rl(50.0) - Rh(2.5), Rl(45.0))).pack::<Base>(),

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.button_sliced_bottom_left.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_a(0.15), ..default() },
                    ..default()
                },

                // Make the background scalable
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),

                // Make it non-obsructable for hit checking (mouse detection)
                PickableBundle::default(),

                // This is required to control our hover animation
                Hover::new().forward_speed(20.0).backward_speed(5.0),

                // This will pipe this hover data to the specified entities
                HoverPipe::new(vec![chevron_left]),

                // This will set the color to red
                BaseColor::new(Color::BEVYPUNK_RED.with_a(0.15)),

                // This will change cursor icon on mouse hover
                HoverCursor::new(CursorIcon::Pointer),

                // This will set hover color to yellow
                HoverColor::new(Color::BEVYPUNK_YELLOW.with_l(0.68)),

                // If we click on this, it will emmit UiClick event
                UiClickEmitter::new(Some(chevron_left)),
            ));

            ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Right"),

                // Add layout
                UiLayout::window().pos((Rl(50.0) + Rh(2.5), Rl(55.0))).size((Rl(50.0) - Rh(2.5), Rl(45.0))).pack::<Base>(),

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.button_sliced_bottom_right.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_a(0.15), ..default() },
                    ..default()
                },

                // Make the background scalable
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),

                // Make it non-obsructable for hit checking (mouse detection)
                PickableBundle::default(),

                // This is required to control our hover animation
                Hover::new().forward_speed(20.0).backward_speed(5.0),

                // This will pipe this hover data to the specified entities
                HoverPipe::new(vec![chevron_right]),

                // This will set the color to red
                BaseColor::new(Color::BEVYPUNK_RED.with_a(0.15)),

                // This will change cursor icon on mouse hover
                HoverCursor::new(CursorIcon::Pointer),

                // This will set hover color to yellow
                HoverColor::new(Color::BEVYPUNK_YELLOW.with_l(0.68)),

                // If we click on this, it will emmit UiClick event
                UiClickEmitter::new(Some(chevron_right)),
            ));

        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

/// System that will react to chevron presses
fn spinner_change_system(mut events: EventReader<UiClick>, mut change: EventWriter<SpinnerChange>, mut query: Query<(&mut Spinner, &Children, Entity)>, mut text: Query<(&SpinnerControl, &mut Text)>) {
    for event in events.read() {
        for (mut spinner, children, entity) in &mut query {
            for child in children {
                if let Ok((spinner_control, mut text)) = text.get_mut(*child) {
                    if spinner_control.chevron_left == event.target  {
                        if spinner.index == 0 { spinner.index = spinner.options.len() - 1 } else { spinner.index -= 1 }
                        text.sections[0].value = spinner.options[spinner.index].clone();
                        change.send(SpinnerChange { target: entity, value: spinner.options[spinner.index].clone() });
                    }
                    if spinner_control.chevron_right == event.target {
                        if spinner.index + 1 == spinner.options.len() { spinner.index = 0 } else { spinner.index += 1 }
                        text.sections[0].value = spinner.options[spinner.index].clone();
                        change.send(SpinnerChange { target: entity, value: spinner.options[spinner.index].clone() });
                    }
                }
            }
        }
    }
}

// #========================#
// #=== COMPONENT PLUGIN ===#

/// Plugin adding all our logic
pub struct SpinnerPlugin;
impl Plugin for SpinnerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiPlugin::<SpinnerUi>::new())

            .add_event::<SpinnerChange>()

            // Add general systems
            .add_systems(Update, spinner_change_system.run_if(on_event::<UiClick>()))
            .add_systems(Update, build_component);
    }
}
