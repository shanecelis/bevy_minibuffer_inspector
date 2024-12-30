//! Demo of bevy_minibuffer integration with bevy-inspector-egui.
//!
//! This composite demo synthesizes several bevy-inspector-egui demos to show
//! how the various inspectors can be made available via minibuffer.
//!
//! The inspector commands are:
//! - world_inspector
//! - resource_inspector
//! - asset_inspector
//! - state_inspector
//! - filter_query_inspector
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
enum AppState {
    #[default]
    A,
    B,
    C,
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    name: String,
    #[inspector(min = 0.0, max = 1.0)]
    option: f32,
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Settings {
    name: String,
    #[inspector(min = 0.0, max = 1.0)]
    option: f32,
}

fn plugin(app: &mut App) {
    app.add_plugins(MinibufferPlugins)
        .add_acts((
            BasicActs::default(),
            inspector::WorldActs::default(),
            inspector::ResourceActs::default()
                .add::<Configuration>()
                .add::<Settings>(),
            inspector::StateActs::default().add::<AppState>(),
            inspector::AssetActs::default().add::<StandardMaterial>(),
            inspector::FilterQueryActs::default()
                .add::<With<Transform>>()
                .add::<With<Mesh3d>>(),
        ))
        .add_systems(Startup, |mut minibuffer: Minibuffer| {
            // minibuffer.message("Type ': Tab' to see the other inspectors.");
            // minibuffer.set_visible(true);
        });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "basic".into(),
                    resolution: [400.0, 400.0].into(),
                    ..Default::default()
                }),
                ..default()
            }),
            plugin,
        ))
        .init_state::<AppState>()
        .register_type::<AppState>()
        .init_resource::<Configuration>()
        .register_type::<Configuration>()
        .init_resource::<Settings>()
        .register_type::<Settings>()
        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(AppState::A),
            set_color(Srgba::hex("8ecae6").unwrap()),
        )
        .add_systems(
            OnEnter(AppState::B),
            set_color(Srgba::hex("d5a220").unwrap()),
        )
        .add_systems(
            OnEnter(AppState::C),
            set_color(Srgba::hex("d2660f").unwrap()),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut minibuffer: Minibuffer,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn set_color(color: impl Into<Color>) -> impl Fn(Commands) {
    let color = color.into();
    move |mut commands: Commands| commands.insert_resource(ClearColor(color))
}
