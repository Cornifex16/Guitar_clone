use bevy::prelude::*;

// Marcador para encontrar el texto del puntaje luego
#[derive(Component)]
pub struct ScoreText;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle { font_size: 40.0, color: Color::WHITE, ..default() },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText, // Etiqueta importante
    ));
}

// Aquí necesitarías un recurso de Score en el módulo 'gameplay' para leerlo
// Por ahora lo dejamos como placeholder visual
pub fn update_score_ui(
    // score: Res<Score>, // Cuando tengas el recurso Score
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in query.iter_mut() {
        // text.sections[0].value = format!("Score: {}", score.value);
    }
}