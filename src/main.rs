//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::time::Duration;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{
    debug::DebugPickingMode,
    events::*,
    picking_core::Pickable,
    prelude::{ListenerInput, On},
    DefaultPickingPlugins, PickableBundle,
};
use bevy_tweening::{lens::*, *};

#[derive(Component)]
struct Card {
    pub pickable: bool,
    pub transform: Transform,
}

fn load_card_materials(
    face_texture: String,
    back_texture: String,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) -> (Handle<StandardMaterial>, Handle<StandardMaterial>) {
    let face_texture = asset_server.load(face_texture);
    let face_material = materials.add(StandardMaterial {
        base_color_texture: Some(face_texture.clone()),
        ..Default::default()
    });

    let face_texture = asset_server.load(back_texture);
    let back_material = materials.add(StandardMaterial {
        base_color_texture: Some(face_texture.clone()),
        ..Default::default()
    });

    (face_material, back_material)
}

// Events

#[derive(Event)]
struct CardHover {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Over>>> for CardHover {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        CardHover {
            entity: event.target,
        }
    }
}

#[derive(Event)]
struct CardOut {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Out>>> for CardOut {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        CardOut {
            entity: event.target,
        }
    }
}

// Main

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_card_hover, handle_card_out))
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            DefaultPickingPlugins,
            TweeningPlugin,
        ))
        .insert_resource(DebugPickingMode::Normal)
        .add_event::<CardHover>()
        .add_event::<CardOut>()
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // circular base
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Circle::new(4.0)),
    //     material: materials.add(Color::WHITE),
    //     transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //     ..default()
    // });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // texture

    let (face_material, back_material) = load_card_materials(
        "card-clubs-1.png".to_string(),
        "card-back4.png".to_string(),
        materials,
        asset_server,
    );

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_rotation(
        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
            * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
            * Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
    );

    // tweening
    commands
        .spawn((
            Card {
                pickable: true,
                transform: transform.clone(),
            },
            PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                transform: transform.clone(),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Over>>::send_event::<CardHover>(),
            On::<Pointer<Out>>::send_event::<CardOut>(),
        ))
        .with_children(|parent| {
            // face
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                    material: face_material,
                    ..default()
                },
                PickableBundle {
                    pickable: Pickable {
                        is_hoverable: false,
                        ..default()
                    },
                    ..default()
                },
            ));
            // back
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                    material: back_material,
                    transform: Transform::IDENTITY
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                    ..default()
                },
                PickableBundle {
                    pickable: Pickable {
                        is_hoverable: false,
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

pub fn handle_card_hover(
    mut commands: Commands,
    mut hover: EventReader<CardHover>,
    mut target_commands: Commands,
    mut query: Query<(Entity, &Card, &mut Transform)>,
) {
    hover.read().for_each(|hover| {
        println!("hovering");
        if let Ok((_, card, mut transform)) = query.get_mut(hover.entity) {
            if card.pickable {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(300),
                    TransformPositionLens {
                        start: transform.translation.clone(),
                        end: card.transform.translation.clone() + Vec3::new(0., 0.0, 0.5),
                    },
                );

                commands.entity(hover.entity).insert(Animator::new(tween));
            }
        }
    });
}

pub fn handle_card_out(
    mut commands: Commands,
    mut out: EventReader<CardOut>,
    mut target_commands: Commands,
    mut query: Query<(Entity, &Card, &mut Transform)>,
) {
    out.read().for_each(|hover| {
        println!("hovering");
        if let Ok((_, card, mut transform)) = query.get_mut(hover.entity) {
            if card.pickable {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(300),
                    TransformPositionLens {
                        start: transform.translation.clone(),
                        end: card.transform.translation.clone(),
                    },
                );

                commands.entity(hover.entity).insert(Animator::new(tween));
            }
        }
    });
}

// |hover, target_commands| {
//     hover.for_each(|hover| {
//         if let Some(card) = target_commands.get_mut::<Card>(hover.entity) {
//             card.pickable = false;
//         }
//     });

//     hover.

//     // tween on hover
//
// }
