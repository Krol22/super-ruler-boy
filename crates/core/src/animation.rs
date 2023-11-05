use bevy::{prelude::*, utils::HashMap };

#[derive(Clone, Debug)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub looping: bool,
    pub fps: usize, 
}

#[derive(Component, Debug)]
pub struct Animator {
    pub animations: HashMap<String, Animation>,
    pub current_animation: String,
    pub prev_animation: String,
    pub current_frame: usize,
    pub counter: f32,
}

pub fn animator_sys (
    mut query: Query<(&mut Animator, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut animator, mut sprite) in query.iter_mut() {
        let mut new_counter = animator.counter + time.delta_seconds();

        let current_animation = animator.current_animation.clone();
        let prev_animation = animator.prev_animation.as_str();

        let animation = animator.animations.get(&current_animation).unwrap().clone();

        let fraction: f32 = 1.0 / (animation.fps as f32);

        if current_animation != prev_animation {
            new_counter = 0.0;
            animator.current_frame = 0;
            sprite.index = animation.frames[0];

            animator.counter = new_counter;
            animator.prev_animation = current_animation.to_string();

            continue;
        }

        if new_counter > fraction {
            let new_frame = (animator.current_frame + 1) % animation.frames.len();

            if animation.looping || new_frame > animator.current_frame {
                animator.current_frame = new_frame;
            } else {
                // If not looping and at the end, hold the last frame
                animator.current_frame = animation.frames.len() - 1;
            }

            sprite.index = animation.frames[animator.current_frame];

            new_counter = 0.0;
        }

        animator.counter = new_counter;
        animator.prev_animation = current_animation.to_string();
    }
}

#[derive(Debug, Default)]
pub struct AnimationPlugin {}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, animator_sys);
    }
}
