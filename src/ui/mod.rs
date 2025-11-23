use bevy::prelude::*;
use crate::states::AppState;

mod menu;
mod hud;
mod layout;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // --- MENU PRINCIPAL ---
            .add_systems(OnEnter(AppState::Menu), menu::setup_menu)
            .add_systems(Update, menu::menu_action_system.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), menu::cleanup_menu)

            // --- JUEGO (HUD + LAYOUT) ---
            // Al entrar al juego, dibujamos el tablero y el HUD
            .add_systems(OnEnter(AppState::Game), (
                layout::setup_game_layout,
                hud::setup_hud
            ))
            // Actualizamos el HUD mientras jugamos
            .add_systems(Update, hud::update_score_ui.run_if(in_state(AppState::Game)))
            
            // Al salir del juego, deberíamos limpiar (aquí simplificado)
            .add_systems(OnExit(AppState::Game), cleanup_game_ui);
    }
}

fn cleanup_game_ui(mut commands: Commands, query: Query<Entity, With<hud::ScoreText>>) {
    // Borrar HUD. El layout y notas se borrarían con la limpieza de gameplay
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}