use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Component)] // Agregamos Component por si acaso
pub struct Note {
    pub time: f64,
    pub lane: usize,
    pub duration: f64,
}

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct SongChart {
    pub name: String,
    pub bpm: f64,
    pub offset: f64,
    pub notes: Vec<Note>,
}