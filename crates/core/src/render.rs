use bevy::{prelude::{Plugin, App, EventReader, Update, AssetEvent, Image, Assets, ResMut, default}, render::{texture::ImageSampler, render_resource::{SamplerDescriptor, AddressMode}}};

#[derive(Debug, Default)]
pub struct RenderPlugin {}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, set_sprites_filter_mode);
    }
}

// Crispy pixel art :chef-kiss:
// I should probably build some solution that will work differently for different textures
pub fn set_sprites_filter_mode(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            if let Some(texture) = assets.get_mut(handle) {
                let sampler_descriptor = SamplerDescriptor {
                    address_mode_u: AddressMode::Repeat,
                    address_mode_v: AddressMode::Repeat,
                    ..default()
                };
                let image_sampler = ImageSampler::Descriptor(sampler_descriptor);

                texture.sampler_descriptor = image_sampler;
            }
        }
    }
}
