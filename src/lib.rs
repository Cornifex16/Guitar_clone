// src/lib.rs

// Declaramos todos los módulos públicos para que main.rs los vea
pub mod states;
pub mod core;
pub mod editor;
pub mod gameplay;
pub mod ui;

// Re-exportamos el AppState para acceder más rápido
pub use states::AppState;

// Opcional: Podrías crear un "Plugin Maestro" aquí que agrupe todos, 
// pero por ahora dejaremos que main.rs los cargue uno por uno para mayor claridad.