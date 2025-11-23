use bevy::prelude::*;

pub fn setup_game_layout(mut commands: Commands) {
    // Posición Y de la línea de golpe (debe coincidir con la lógica de gameplay)
    let hit_line_y = -250.0; 

    // Dibujar los 5 receptores estáticos
    let colors = [Color::GREEN, Color::RED, Color::YELLOW, Color::BLUE, Color::rgb(1.0, 0.5, 0.0)];

    for i in 0..5 {
        let x_pos = (i as f32 * 50.0) - 100.0;

        // El círculo "objetivo"
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(x_pos, hit_line_y, -1.0), // Z=-1 para estar detrás de las notas
            sprite: Sprite {
                color: colors[i].with_a(0.3), // Semi-transparente
                custom_size: Some(Vec2::new(50.0, 50.0)), // Un poco más grande que la nota
                ..default()
            },
            ..default()
        });
        
        // La línea vertical del carril
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(x_pos, 0.0, -2.0), // Aún más atrás
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(2.0, 1000.0)), // Línea larga
                ..default()
            },
            ..default()
        });
    }
}