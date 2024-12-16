use bevy::prelude::*;
use rand::Rng;

use super::componets::*;
use crate::game::ball::resources::Game;

pub fn win_cat(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
){
    if game.win{

        let random_cat = rand::thread_rng().gen_range(0..2);

        let cat_array = ["PirateCats/PirateCat1.png", "PirateCats/PirateCat2.png"];
        let cat_handle = asset_server.load(cat_array[random_cat]);

        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        
        let cat_indices: CatIndices;
      if random_cat == 0{
        cat_indices = CatIndices{first: 0, last: 6};
      }
      else{
        cat_indices = CatIndices{first: 0, last: 2};
      }

        
        commands.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),  
                height: Val::Percent(100.0), 
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,  
                align_items: AlignItems::Center,          
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0), 
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            z_index: ZIndex::Global(-1),
            ..default()
        },
        CatUI
    ))
        .with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(50.0), 
                    height: Val::Percent(50.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center, 
                    align_items: AlignItems::Center,       
                    border: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.361, 0.251, 0.200)),
                border_radius: BorderRadius::new(
                    Val::Px(40.),
                    Val::Px(40.),
                    Val::Px(40.),
                    Val::Px(40.),
                ),
                border_color: BorderColor(Color::srgb(0., 0., 0.)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0), 
                    ..default()
                },
                
                z_index: ZIndex::Global(-1),
                ..default()
                
            },
            CatUI
        ))
            .with_children(|parent| {
                parent.spawn((ImageBundle {
                    style: Style {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage{
                        texture: cat_handle,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    transform: Transform {
                        scale: Vec3::splat(8.0),
                        translation: Vec3::new(0.0, 0.0, 1.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(1), 
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
                CatUI,
                cat_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Cat
            ));

            parent.spawn((NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            },
            CatUI
        ));
        

            parent.spawn(
                (TextBundle{
                    style: Style{
                        margin: UiRect {
                            top: Val::Px(100.0),
                            ..default()
                        },
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section("YOU WIN! PRESS ENTER TO KEEP PLAYING", TextStyle::default()),
                    z_index: ZIndex::Global(2),
                    ..default()
                },
            CatUI));
            });
        });


        commands.spawn(AudioBundle {
            source: asset_server.load("audio/catMeow.mp3"),
            settings: PlaybackSettings{
                mode: bevy::audio::PlaybackMode::Despawn,
              ..default()
            },
        });
        game.win = false;
    }
}


pub fn depawn_cat(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    cat_query: Query<Entity, With<CatUI>>
){
    if keyboard_input.just_pressed(KeyCode::Enter){

        for cat_ui in cat_query.iter() {
            commands.entity(cat_ui).despawn();
        }
    }
}

pub fn animate_cat(
    time: Res<Time>,
    mut query: Query<(&CatIndices, &mut AnimationTimer, &mut TextureAtlas), With<Cat>>
)
{   
    for(indices, mut timer, mut atlas) in &mut query{
        timer.0.tick(time.delta());
        if timer.0.just_finished(){
            atlas.index = if atlas.index == indices.last{
                indices.first
            }else{
                atlas.index+1
            }
        }
    }
}