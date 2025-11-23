use bevy::prelude::*;

pub fn editor_visuals(
    mut gizmos: Gizmos,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Dibujar 5 carriles estáticos en el suelo
    for i in 0..5 {
        let x_pos = (i as f32 * 50.0) - 100.0; // Separación simple
        gizmos.line_2d(
            Vec2::new(x_pos, -300.0), 
            Vec2::new(x_pos, 300.0), 
            Color::GRAY
        );
    }

    // Dibujar círculo si presionas tecla (Feedback instantáneo)
    let keys = [KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyL];
    let colors = [Color::GREEN, Color::RED, Color::YELLOW, Color::BLUE, Color::rgb(1.0, 0.5, 0.0)]; // Naranja

    for (i, key) in keys.iter().enumerate() {
        if input.pressed(*key) {
            let x_pos = (i as f32 * 50.0) - 100.0;
            gizmos.circle_2d(Vec2::new(x_pos, -250.0), 20.0, colors[i]);
        }
    }
}