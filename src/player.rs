use std::f32::consts::PI;

use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
// use bevy_rapier2d::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                // .with_system(spawn_player)
                // .with_system(spawn_ground)
                // .with_system(spawn_player)
                // .with_system(spawn_camera)
                // .with_system(spawn_wheel_bot),
                .with_system(setup)
        )
        // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(PhysicsPlugin::default())
        // .insert_resource(Gravity::from(Vec2::new(0.0, -29.8)))
        ;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            // transform: Transform::default().with_scale(Vec3::splat(128.)),
            transform: Transform {
                translation: Vec3::new(0., -200., 0.),
                scale: Vec3::new(512., 32., 0.),
                ..default()
            },
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(512., 32., 0.) / 2.0,
            border_radius: None,
        });
}

fn spawn_wheel_bot(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_map.clone(),
            transform: Transform::from_scale(Vec3::new(2.5, 2.5, 0.)),
            ..default()
        })
        .insert(RigidBody::Dynamic);
        // .insert(CollisionShape::Cuboid { half_extends: Vec3::new(13., 13., 0.) / 2., border_radius: None});
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            // transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            // transform: Transform::from_scale(Vec3::new(5., 5., 0.)), 
            transform: Transform {
                translation: Vec3::new(-200., 300., 0.),
                scale: Vec3::new(2., 2., 0.),
                rotation: Quat::from_rotation_x(PI/2.)
                // rotation: Quat::from_rotation_y(PI/2./2.)
                // ..default()
                // rotation:Quat::from_rotation_z(0.)
            },
            ..default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid { half_extends: Vec3::new(2., 8., 0.) / 2., border_radius: None})
        // .insert(Acceleration {
        //     linear: Vec3::new(rand::thread_rng().gen_range(0.0..9.8), 0., 0.),
        //     angular: Vec3::new(rand::thread_rng().gen_range(0.1..1.0), 0., 0.).into()
        // })
        // .insert(Acceleration::from_linear(Vec3::X * rand::thread_rng().gen_range(9.0..9.8)))
        // .insert(Velocity{})
        // .insert(Acceleration::from_linear(Vec3::X * 9.8))
        // .insert(Acceleration::from_angular(Vec3::new(9.5, 0., 0.).into()))
        // .insert_bundle(TransformBundle::from(Transform::from_xyz(-300.0, 0.0, 0.0)))
        // .insert(Velocity::from_linear(Vec3::X * 300.0))
        // .insert(Velocity{linvel: Vec2::new(100.0, 0.), angvel: 5.})
        // .insert(MassProperties {
        //     local_center_of_mass: Vec2::new(1.0, 2.0),
        //     mass: 1.0,
        //     principal_inertia: 0.5,
        // })
        // .insert(Collider::ball(10.0))
        // .insert(ColliderMassProperties::Density(2.0))
        // .insert(ExternalForce {
        //     force: Vec2::new(1000000.0, 0.0),
        //     torque: 0.,
        // })
        // .insert(GravityScale(0.))
        // .insert(Sleeping::disabled())
        // .insert(Ccd::enabled())
        ;
}

fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // if actions.player_movement.is_none() {
    //     return;
    // }
    let speed = 150.;
    // let movement = Vec3::new(
    //     actions.player_movement.unwrap().x * speed * time.delta_seconds(),
    //     actions.player_movement.unwrap().y * speed * time.delta_seconds(),
    //     0.,
    // );
    let movement = Vec3::new(
        // 1. * speed * time.delta_seconds() * rand::thread_rng().gen_range(-5.0..4.9),
        5. * speed * time.delta_seconds(),
        // 1. * speed * time.delta_seconds() * rand::thread_rng().gen_range(-1.0..1.001),
        // 1. * speed * time.delta_seconds(),
        0.,
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        if (player_transform.translation.x > 500. || player_transform.translation.y < -500.) {
            player_transform.translation.x = -500.;
            player_transform.translation.y = 0.;
        }
        if (player_transform.translation.y > 250. || player_transform.translation.y < -250.) {
            player_transform.translation.y = 0.;
        }
        // player_transform.translation += movement;
    }
}
