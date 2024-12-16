use bevy::prelude::*;

#[derive(Component)]
pub struct CatUI;


#[derive(Component)]
pub struct Cat;

#[derive(Component)]
pub struct CatIndices{
    pub first: usize,
    pub last: usize
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);