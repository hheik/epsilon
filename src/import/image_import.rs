use bevy::{
    prelude::*,
    render::{render_resource::AddressMode, texture::ImageSampler},
    utils::HashMap,
};

#[derive(Default)]
pub struct ImageImporter {
    import_queue: HashMap<Handle<StandardMaterial>, ImageImportData>,
}

impl ImageImporter {
    pub fn new() -> Self {
        ImageImporter::default()
    }

    pub fn queue_import(&mut self, handle: Handle<StandardMaterial>, import_data: ImageImportData) {
        self.import_queue.insert(handle, import_data);
    }
}

pub struct ImageImportData {
    sampler: ImageSampler,
}

pub struct ImageImportPlugin;

impl Plugin for ImageImportPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageImporter::new())
            .add_system(event_handler)
            .add_system(import_flusher);
    }
}

fn event_handler(
    mut events: EventReader<AssetEvent<StandardMaterial>>,
    // mut a_events: EventReader<bevy::prelude::>,
    mut image_importer: ResMut<ImageImporter>,
) {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            let mut sampler = ImageSampler::nearest_descriptor();
            sampler.address_mode_u = AddressMode::Repeat;
            sampler.address_mode_v = AddressMode::Repeat;
            sampler.address_mode_w = AddressMode::Repeat;
            let data = ImageImportData {
                sampler: ImageSampler::Descriptor(sampler),
            };
            image_importer.queue_import(handle.clone(), data);
        }
    }
}

fn import_flusher(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut image_importer: ResMut<ImageImporter>,
) {
    let handles: Vec<Handle<StandardMaterial>> =
        image_importer.import_queue.keys().cloned().collect();
    for handle in &handles {
        let material = match materials.get_mut(handle) {
            Some(material) => material,
            None => return,
        };
        let data = image_importer
            .import_queue
            .remove(&handle)
            .expect("No import data for given handle");

        if let Some(image) = &mut material.base_color_texture {
            if let Some(image) = &mut images.get_mut(image) {
                image.sampler_descriptor = data.sampler;
            }
        }
    }
}
