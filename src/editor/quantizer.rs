use crate::core::Note;

// Función pura: Transforma tiempos sucios en Notas limpias
pub fn quantize_notes(raw_events: &Vec<(f64, usize)>, bpm: f64) -> Vec<Note> {
    let mut clean_notes = Vec::new();
    
    // 60 segundos / BPM = duración de un beat (negra)
    let beat_duration = 60.0 / bpm;
    
    // Dividimos el beat en 4 (semicorcheas / 16th notes)
    // Esta es la "rejilla" magnética
    let snap_interval = beat_duration / 4.0; 

    for (raw_time, lane) in raw_events {
        // FORMULA MÁGICA: Redondear al intervalo más cercano
        // Ejemplo: Si el intervalo es 0.25s y tocas en 0.26s, te mueve a 0.25s
        let snapped_time = (raw_time / snap_interval).round() * snap_interval;

        clean_notes.push(Note {
            time: snapped_time,
            lane: *lane,
            duration: 0.0, // Por defecto nota corta
        });
    }
    
    clean_notes
}