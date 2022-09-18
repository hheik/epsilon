use bevy::{
    prelude::*,
    render::{render_resource::AddressMode, texture::ImageSampler},
    utils::HashMap,
};

#[derive(Default)]
pub struct ImageImporter {
    import_queue: HashMap<Handle<Image>, ImageImportData>,
}

impl ImageImporter {
    pub fn new() -> Self {
        ImageImporter::default()
    }

    pub fn queue_import(&mut self, handle: Handle<Image>, import_data: ImageImportData) {
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
    mut events: EventReader<AssetEvent<Image>>,
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

fn import_flusher(mut images: ResMut<Assets<Image>>, mut image_importer: ResMut<ImageImporter>) {
    let handles: Vec<Handle<Image>> = image_importer.import_queue.keys().cloned().collect();
    for handle in &handles {
        let mut image = match images.get_mut(handle) {
            Some(image) => image,
            None => return,
        };
        let data = image_importer
            .import_queue
            .remove(&handle)
            .expect("No import data for given handle");
        image.sampler_descriptor = data.sampler;
    }

    for (_, image) in images.iter_mut() {
        let mut sampler = ImageSampler::nearest_descriptor();
        sampler.address_mode_u = AddressMode::Repeat;
        sampler.address_mode_v = AddressMode::Repeat;
        sampler.address_mode_w = AddressMode::Repeat;
        image.sampler_descriptor = ImageSampler::Descriptor(sampler);
    }
}
