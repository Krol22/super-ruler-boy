use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    frames: Vec<usize>,
}

#[derive(Component)]
pub struct ParticleEmitter {
    pub frames: Vec<usize>,
    pub particle_lifetime: f32,
    pub handle: Handle<TextureAtlas>,
    pub spawning: bool,
    pub spawn_timer: Timer, 
}

fn spawn_test_emitter (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite.png");

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(4.0, 4.0),
        12,
        5,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Transform::from_xyz(200.0, 200.0, 0.0),
        ParticleEmitter {
            frames: vec![48, 49, 50, 51, 52],
            particle_lifetime: 1.0,
            handle: texture_atlas_handle,
            spawning: true,
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    ));
}

fn particle_update_sys (
    mut query: Query<(Entity, &mut Particle, &mut TextureAtlasSprite)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut particle, mut texture_atlas_sprite) in query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let frame = particle.lifetime.percent() * particle.frames.len() as f32;
        if frame.floor() as usize >= particle.frames.len() {
            continue;
        }

        texture_atlas_sprite.index = particle.frames[frame.floor() as usize];
    } 
}

fn emitter_sys (
    mut commands: Commands,
    mut query: Query<(&Transform, &mut ParticleEmitter)>,
    timer: Res<Time>,
) {
    for (transform, mut emitter) in query.iter_mut() {
        if !emitter.spawning {
            emitter.spawn_timer.set_elapsed(Duration::from_secs(0));
            continue;
        }

        emitter.spawn_timer.tick(timer.delta());

        if !emitter.spawn_timer.finished() {
            continue;
        }

        if emitter.spawn_timer.mode() == TimerMode::Once {
            emitter.spawning = false;
        }

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: emitter.handle.clone(),
                sprite: TextureAtlasSprite::new(emitter.frames[0]),
                transform: Transform::from_xyz(transform.translation.x, transform.translation.y, transform.translation.z),
                ..default()
            },
            Particle {
                lifetime: Timer::from_seconds(emitter.particle_lifetime, TimerMode::Once),
                frames: emitter.frames.to_vec(),
            }
        ));
    }
}

#[derive(Debug, Default)]
pub struct ParticlePlugin {}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, spawn_test_emitter)
            .add_systems(Update, emitter_sys)
            .add_systems(Update, particle_update_sys);
    }
}
