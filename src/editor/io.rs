use bevy::prelude::*;
use std::fs::File;
use std::io::Write;

use crate::core::{SongChart, Conductor};
use crate::editor::recorder::RecordingSession;
use crate::editor::quantizer::quantize_notes;

pub fn save_song_system(
    input: Res<ButtonInput<KeyCode>>,
    session: Res<RecordingSession>,
    conductor: Res<Conductor>,
    // Opcional: Podríamos escribir en el recurso SongChart global también
) {
    if input.just_pressed(KeyCode::Enter) {
        println!("Procesando y guardando...");

        // 1. Limpiar los datos humanos (Quantize)
        // Asumimos un BPM fijo por ahora, luego vendrá del SongChart
        let clean_notes = quantize_notes(&session.raw_events, conductor.bpm);

        // 2. Crear la estructura final
        let final_chart = SongChart {
            name: "Mi Canción Nueva".to_string(),
            bpm: conductor.bpm,
            offset: 0.0,
            notes: clean_notes,
        };

        // 3. Serializar a JSON
        // .unwrap() es peligroso en producción, pero bien para prototipos
        let json_str = serde_json::to_string_pretty(&final_chart).unwrap();

        // 4. Escribir a disco
        let path = "assets/charts/output.json";
        let mut file = File::create(path).expect("No se pudo crear el archivo");
        file.write_all(json_str.as_bytes()).expect("Error escribiendo");

        println!("¡Guardado exitoso en {}!", path);
    }
}