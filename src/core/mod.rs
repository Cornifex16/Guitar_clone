use bevy::prelude::*;

// 1. Declarar módulos públicos
pub mod audio;
pub mod chart;

// 2. Re-exportar para usar crate::core::Note directamente
pub use audio::*;
pub use chart::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Ahora Rust ya sabe qué es SongChart y Conductor porque hicimos 'use chart::*;' arriba
            .init_resource::<SongChart>()
            .init_resource::<Conductor>()
            .add_systems(Update, audio::update_song_time);
    }
}