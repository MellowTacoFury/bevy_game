use bevy::prelude::*;
use bevy::window::PrimaryWindow;
fn main()
{
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_player)
    .run();
}


//make a player
#[derive(Component)]
struct Player
{

}
fn spawn_player(
    mut commands: Commands,//for spawning entity
    windowQuery: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
    assetServer: Res<AssetServer>//load the pngs
)
{
    //let the window of type &Window 
    //be the only query that is window with PW, unwrap to give it to us
    let window: &Window = windowQuery.single().unwrap();

    //spawn an entity with -> (Sprite{},Transform(), Player{})
    //src\assets\sprites\ball_blue_large.png
    commands.spawn(
        (
            Sprite{
                image: assetServer.load("sprites\\ball_blue_large.png"),
                ..Default::default()
            },
            Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            Player{}
        )
    );


}


//make a camera
fn spawn_camera(
    mut commands: Commands,//for spawning entity
    windowQuery: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
)
{
    //let the window of type &Window 
    //be the only query that is window with PW, unwrap to give it to us
    let window: &Window = windowQuery.single().unwrap();

    //Spawn defalt camera, in the center of the screen
    commands.spawn(
        (
        Camera2d::default(),
        Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        )
    );
}