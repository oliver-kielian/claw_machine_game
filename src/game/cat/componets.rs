use bevy::prelude::*;


///Struct for the CatUI Componets, allows for each despwan
#[derive(Component)]
pub struct CatUI;

///Struct for the Cat Componets, allows for animation
#[derive(Component)]
pub struct Cat;

///Struct that holds the first and last indices of the sprite sheet
#[derive(Component)]
pub struct CatIndices{
    pub first: usize,
    pub last: usize
}


///Struct that controls the animation timing per frame
#[derive(Component)]
pub struct AnimationTimer(pub Timer);