use bevy::prelude::*;
use crate::states::AppState;

// IMPORTANTE: 'pub mod' hace que el contenido sea visible
pub mod recorder;
pub mod quantizer;
pub mod io;
pub mod visualizer;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Ahora sí encontrará recorder::RecordingSession
            .init_resource::<recorder::RecordingSession>()

            .add_systems(Update, (
                recorder::record_input,
                visualizer::editor_visuals,
                io::save_song_system,
            ).run_if(in_state(AppState::Editor)))
            
            .add_systems(OnExit(AppState::Editor), cleanup_editor);
    }
}

fn cleanup_editor(mut session: ResMut<recorder::RecordingSession>) {
    session.raw_events.clear();
    println!("Editor limpiado.");
}