use bevy::prelude::*;
use crate::core::{Note, Conductor};

pub fn move_notes_system(
    mut commands: Commands,
    conductor: Res<Conductor>,
    // Consultamos todas las entidades que tengan componente Note y Transform
    mut query: Query<(Entity, &Note, &mut Transform)>,
) {
    let speed = 400.0; // Píxeles por segundo
    let hit_line_y = -250.0; // Donde está la línea de golpe

    for (entity, note, mut transform) in query.iter_mut() {
        // FÓRMULA CLAVE:
        // (TiempoObjetivo - TiempoActual) * Velocidad
        // Si faltan 1s: (1.0) * 400 = +400px (arriba de la línea)
        // Si pasaron 0s: (0.0) * 400 = 0px (en la línea)
        // Si ya pasó -0.5s: (-0.5) * 400 = -200px (debajo de la línea)
        let time_diff = note.time - conductor.song_position;
        
        transform.translation.y = hit_line_y + (time_diff as f32 * speed);

        // Lógica de MISS (Si la nota bajó demasiado)
        // -0.2s significa que ya pasó 200ms después de la línea
        if time_diff < -0.2 {
            println!("MISS! Se te pasó una nota");
            commands.entity(entity).despawn(); 
            // Aquí restarías vida o combo
        }
    }
}