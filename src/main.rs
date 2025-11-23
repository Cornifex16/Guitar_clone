// src/main.rs
use bevy::prelude::*;

// IMPORTANTE: Reemplaza 'guitar_rust' por el nombre de tu proyecto en Cargo.toml
use guitar_rust::{
    core::CorePlugin,
    editor::EditorPlugin,
    gameplay::GamePlugin,
    ui::UiPlugin,
    AppState,
};

fn main() {
    App::new()
        // 1. Configuración Básica de Bevy (Ventana, etc.)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Hero - Bevy Edition".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))

        // 2. Inicializar Estado Global
        .init_state::<AppState>()

        // 3. Configuración Inicial (Cámara)
        .add_systems(Startup, setup_camera)

        // 4. Cargar NUESTROS Plugins (La lógica que escribimos)
        .add_plugins((
            CorePlugin,   // Audio y Datos
            UiPlugin,     // Menús y HUD
            EditorPlugin, // Grabadora
            GamePlugin,   // Juego
        ))

        // 5. ¡Correr!
        .run();
}

// Sistema simple para crear la cámara 2D
fn setup_camera(mut commands: Commands) {
    // Sin esto, no se renderiza nada en pantalla
    commands.spawn(Camera2dBundle::default());
}