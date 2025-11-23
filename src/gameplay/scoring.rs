use bevy::prelude::*;
use crate::core::{Note, Conductor};

pub fn check_hits_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    conductor: Res<Conductor>,
    // Necesitamos saber qué notas están vivas
    note_query: Query<(Entity, &Note)>,
) {
    let keys = [KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyL];
    let hit_window = 0.15; // 150ms de margen de error (Ventana de golpe)

    for (lane_index, key) in keys.iter().enumerate() {
        if input.just_pressed(*key) {
            let mut hit_something = false;

            for (entity, note) in note_query.iter() {
                // 1. ¿Es del mismo carril?
                if note.lane != lane_index { continue; }

                // 2. ¿Está en tiempo?
                // abs() nos da la diferencia absoluta (sea antes o después)
                let diff = (note.time - conductor.song_position).abs();

                if diff <= hit_window {
                    // ¡GOLPE EXITOSO!
                    println!("HIT! Diferencia: {:.3}s", diff);
                    
                    // Eliminamos la nota visual
                    commands.entity(entity).despawn();
                    
                    hit_something = true;
                    
                    // Solo podemos golpear una nota por pulsación, así que rompemos el bucle
                    // (Esto evita golpear 2 notas muy pegadas con un solo click)
                    break; 
                }
            }

            if !hit_something {
                println!("OVERSTRUM! Tocaste sin nota (Rompe combo)");
            }
        }
    }
}