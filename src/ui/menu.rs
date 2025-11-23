use bevy::prelude::*;
use crate::states::AppState;

// Componente marcador para identificar el nodo raíz del menú y borrarlo fácil
#[derive(Component)]
pub struct MainMenuRoot;

pub fn setup_menu(mut commands: Commands) {
    // Nodo Raíz (Contenedor gris centrado)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0), // Espacio entre botones
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            // Título
            parent.spawn(TextBundle::from_section(
                "RUST HERO",
                TextStyle { font_size: 60.0, color: Color::WHITE, ..default() },
            ));

            // Botón Jugar
            spawn_button(parent, "Jugar Cancion", AppState::Game);
            
            // Botón Editor
            spawn_button(parent, "Modo Editor", AppState::Editor);
        });
}

// Helper para no repetir código de botones
fn spawn_button(parent: &mut ChildBuilder, text: &str, target_state: AppState) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                ..default()
            },
            // Guardamos el estado al que queremos ir dentro del botón (truco útil)
            MenuButtonAction(target_state), 
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(text, TextStyle { font_size: 30.0, ..default() }));
        });
}

// Componente para guardar qué hace cada botón
#[derive(Component)]
pub struct MenuButtonAction(AppState);

// SISTEMA: Detectar Clics
pub fn menu_action_system(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(action.0); // Cambiamos al estado guardado en el botón
        }
    }
}

// SISTEMA: Limpieza
pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}