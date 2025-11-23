use bevy::prelude::*;
use crate::core::{SongChart, Conductor, Note};

// Recurso para saber por cuál nota vamos
#[derive(Resource, Default)]
pub struct SpawnCursor {
    pub next_note_index: usize,
}

pub fn spawn_notes_system(
    mut commands: Commands,
    chart: Res<SongChart>,
    conductor: Res<Conductor>,
    mut cursor: ResMut<SpawnCursor>,
    // Assets para dibujar (en un juego real cargarías texturas)
    // asset_server: Res<AssetServer>, 
) {
    // CONFIG: ¿Cuántos segundos antes de sonar debe aparecer la nota?
    let spawn_ahead_time = 2.0; 

    // Revisamos las notas futuras
    while cursor.next_note_index < chart.notes.len() {
        let note_data = &chart.notes[cursor.next_note_index];

        // Si la nota debe sonar en 10s, y la canción va por 8.1s, 
        // entra en la ventana de (10 - 8.1 = 1.9s) < 2.0s -> SPAWN!
        if note_data.time - conductor.song_position < spawn_ahead_time {
            
            // Posición X basada en el carril (0, 1, 2, 3, 4)
            let x_pos = (note_data.lane as f32 * 50.0) - 100.0;

            // Crear la entidad visual
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x_pos, 500.0, 0.0), // Empieza arriba
                    sprite: Sprite {
                        color: get_lane_color(note_data.lane),
                        custom_size: Some(Vec2::new(40.0, 20.0)),
                        ..default()
                    },
                    ..default()
                },
                // AÑADIMOS EL COMPONENTE DE DATOS (Importante para moverla luego)
                note_data.clone(), 
            ));

            cursor.next_note_index += 1;
        } else {
            // Si la siguiente nota está muy lejos, dejamos de revisar por este frame
            break;
        }
    }
}

pub fn get_lane_color(lane: usize) -> Color {
    match lane {
        0 => Color::GREEN,
        1 => Color::RED,
        2 => Color::YELLOW,
        3 => Color::BLUE,
        _ => Color::rgb(1.0, 0.5, 0.0), // Naranja
    }
}