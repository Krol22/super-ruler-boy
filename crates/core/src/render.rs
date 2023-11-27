use bevy::{prelude::{Plugin, App, EventReader, Update, AssetEvent, Image, Assets, ResMut, default, AssetServer, Res}, render::{texture::ImageSampler, render_resource::{SamplerDescriptor, AddressMode}}};

#[derive(Debug, Default)]
pub struct RenderPlugin {}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, set_sprites_filter_mode);
    }
}

// Crispy pixel art :chef-kiss:
// I should probably build some solution that will work differently for different textures/
pub fn set_sprites_filter_mode(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            if let Some(texture) = assets.get_mut(handle) {
                // Determine the appropriate address mode based on the handle path
                let address_mode = match asset_server.get_handle_path(handle)
                    .and_then(|handle_path| {
                        handle_path.path().to_str().map(String::from)
                    })
                {
                    Some(ref path) if path == "sprites/ruler_extension_part.png" => AddressMode::Repeat,
                    _ => AddressMode::ClampToEdge,
                };

                // Set the sampler descriptor
                let sampler_descriptor = SamplerDescriptor {
                    address_mode_u: address_mode,
                    address_mode_v: address_mode,
                    ..default()
                };
                texture.sampler_descriptor = ImageSampler::Descriptor(sampler_descriptor);
            }
        }
    }
}
