use bevy::prelude::*;
use bevy::window::PrimaryWindow;


//consts
pub const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;//pixels I think
const STAR_SIZE: f32 = 32.0;//pixels I think

const NUMBER_OF_STARS:usize = 10;


fn main()
{
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(SpawnersPlugin)
    //updates
    .add_systems(Update, player_movement)
    .add_systems(Update, confine_player)
    .run();
}

struct SpawnersPlugin;
impl  Plugin for SpawnersPlugin {
    fn build(&self, app: &mut App) {
        //startups
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, spawn_player);
        app.add_systems(Startup, spawn_stars);
    }
}


//componetns
#[derive(Component)]
struct Player{}

#[derive(Component)]
struct Enemy{}

//Spawners
fn spawn_player(
    mut commands: Commands,//for spawning entity
    window_query: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
    asset_server: Res<AssetServer>//load the pngs
)
{
    //let the window of type &Window 
    //be the only query that is window with PW, unwrap to give it to us
    let window: &Window = window_query.single().unwrap();

    //spawn an entity with -> (Sprite{},Transform(), Player{})
    //src\assets\sprites\ball_blue_large.png
    commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/ball_blue_large.png"),
                ..Default::default()
            },
            Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            Player{}
        )
    );


}

fn spawn_camera(
    mut commands: Commands,//for spawning entity
    window_query: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
)
{
    //let the window of type &Window 
    //be the only query that is window with PW, unwrap to give it to us
    let window: &Window = window_query.single().unwrap();

    //Spawn defalt camera, in the center of the screen
    commands.spawn(
        (
        Camera2d::default(),
        Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        )
    );
}

fn spawn_stars(
    mut commands: Commands,//for spawning entity
    window_query: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
    asset_server: Res<AssetServer>//load the pngs
)
{
    
    let window: &Window = window_query.single().unwrap();
    //loop through the enemies
    for _ in 0..NUMBER_OF_STARS
    {
        let half_size = STAR_SIZE/2.0;

        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        //lock to the screen
        let clamped_x = random_x.clamp(half_size, window.width() - half_size);
        let clamped_y = random_y.clamp(half_size, window.height() - half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..Default::default()
            },
            Transform::from_xyz(clamped_x, clamped_y, 0.0),
            Enemy {},
        ));
    }
}



//move the player
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    //gets us the player's transform, or an error
    if let Ok(mut transform) = player_query.single_mut()
    {
        let mut direction = Vec3::ZERO;

        //get keyboard input
        if keyboard_input.pressed(KeyCode::ArrowLeft)
        {
            direction += Vec3::new(-10.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight)
        {
            direction += Vec3::new(10.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp)
        {
            direction += Vec3::new(0.0, 10.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown)
        {
            direction += Vec3::new(0.0, -10.0, 0.0);
        }

        //normalize it
        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        //move
        transform.translation += direction * PLAYER_SPEED * time.delta_secs();


    }
}


//need the player and window to keep the player in the window
fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
){
    if let Ok(mut transform) = player_query.single_mut()
    {
        //yay we have the player transform
        
        let window: &Window = window_query.single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE /2.0;

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


