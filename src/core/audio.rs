use bevy::prelude::*;

// IMPORTANTE: 'pub' hace que sea visible desde fuera
#[derive(Resource, Default)]
pub struct Conductor {
    pub song_position: f64,
    pub bpm: f64,
    pub is_playing: bool,
    pub offset: f64,
}

// IMPORTANTE: 'pub' delante de fn
pub fn update_song_time(
    time: Res<Time>,
    mut conductor: ResMut<Conductor>,
) {
    if conductor.is_playing {
        conductor.song_position += time.delta_seconds_f64();
    }
}