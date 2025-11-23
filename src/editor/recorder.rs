use bevy::prelude::*;
use crate::core::Conductor;

// Recurso temporal: Aquí guardamos lo que tocas "en sucio"
#[derive(Resource, Default)]
pub struct RecordingSession {
    // Tupla: (Tiempo exacto, Indice de Carril 0-4)
    pub raw_events: Vec<(f64, usize)>,
}

// Sistema: Escucha el teclado
pub fn record_input(
    input: Res<ButtonInput<KeyCode>>, // En versiones nuevas de Bevy es ButtonInput
    conductor: Res<Conductor>,
    mut session: ResMut<RecordingSession>,
) {
    // Mapeo de teclas: A=Verde(0), S=Rojo(1), etc.
    let keys = [KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyL];

    for (lane, key) in keys.iter().enumerate() {
        if input.just_pressed(*key) {
            // Guardamos el momento exacto del Conductor
            session.raw_events.push((conductor.song_position, lane));
            
            println!("Nota grabada: {:.3}s en carril {}", conductor.song_position, lane);
        }
    }
}