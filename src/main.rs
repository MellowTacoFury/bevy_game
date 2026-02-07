use bevy::prelude::*;
use bevy::window::PrimaryWindow;


//consts
pub const PLAYERSPEED: f32 = 500.0;
const PLAYERSIZE: f32 = 64.0;//pixels I think


fn main()
{
    App::new()
    .add_plugins(DefaultPlugins)
    //startups
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_player)
    //updates
    .add_systems(Update, player_movement)
    .add_systems(Update, confine_player)
    .run();
}


//make a player
#[derive(Component)]
struct Player{}

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


//move the player
fn player_movement(
    keyboardInput: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    //gets us the player's transform, or an error
    if let Ok(mut transform) = player_query.single_mut()
    {
        let mut direction = Vec3::ZERO;

        //get keyboard input
        if keyboardInput.pressed(KeyCode::ArrowLeft)
        {
            direction += Vec3::new(-10.0, 0.0, 0.0);
        }
        if keyboardInput.pressed(KeyCode::ArrowRight)
        {
            direction += Vec3::new(10.0, 0.0, 0.0);
        }
        if keyboardInput.pressed(KeyCode::ArrowUp)
        {
            direction += Vec3::new(0.0, 10.0, 0.0);
        }
        if keyboardInput.pressed(KeyCode::ArrowDown)
        {
            direction += Vec3::new(0.0, -10.0, 0.0);
        }

        //normalize it
        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        //move
        transform.translation += direction * PLAYERSPEED * time.delta_secs();


    }
}


//need the player and window to keep the player in the window
fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    windowQuery: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
){
    if let Ok(mut transform) = player_query.single_mut()
    {
        //yay we have the player transform
        
        let window: &Window = windowQuery.single().unwrap();

        let half_player_size: f32 = PLAYERSIZE /2.0;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        //grab this
        let mut translation = transform.translation;

        //lock the x
        if translation.x < x_min{
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }
        //lock the y
        if translation.y < y_min{
            translation.y = y_min;
        }else if translation.y > y_max{
            translation.y = y_max;
        }

        //put it back
        transform.translation = translation;

    }
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


