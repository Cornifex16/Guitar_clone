use bevy::prelude::*;
use crate::states::AppState;

pub mod spawner;
pub mod movement;
pub mod scoring;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Inicializar el cursor en 0
            .init_resource::<spawner::SpawnCursor>()

            // AL ENTRAR: Resetear cursor y cosas necesarias
            .add_systems(OnEnter(AppState::Game), setup_game)

            // UPDATE: Solo corren mientras jugamos
            .add_systems(Update, (
                spawner::spawn_notes_system,
                movement::move_notes_system,
                scoring::check_hits_system,
            ).run_if(in_state(AppState::Game)))

            // AL SALIR: Borrar todas las notas que queden
            .add_systems(OnExit(AppState::Game), cleanup_game);
    }
}

fn setup_game(mut cursor: ResMut<spawner::SpawnCursor>) {
    cursor.next_note_index = 0;
    println!("Juego iniciado");
}

fn cleanup_game(
    mut commands: Commands,
    query: Query<Entity, With<crate::core::Note>>,
) {
    // Borrar todas las entidades que tengan el componente Note
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    println!("Limpieza del juego completada");
}