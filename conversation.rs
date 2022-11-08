use std::string;

use bevy::{
	prelude::*,
	text::Text2dBounds,
};
use super::ConvInputEvent;
use super::ConvLossEvent;
use super::ConvWinEvent;

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct DialogueBox;

#[derive(Component)]
pub struct UserInput;

#[derive(Component)]
pub struct EnemyDialogue;

#[derive(Component)]
pub struct Button;

/*#[derive(Component)]
pub struct NiceResponses{

}*/

// 0 - start (enemy prompt, wait for player prompt)
// 1 - after player first response, fetch ai response
// 2 - after player second response, fetch ai response
// etc.. 
// FINAL TURN - after player final response, return fight or not
const MAX_TURNS: i32 = 4;
const START_TURN: i32 = 0;
static mut CUR_TURN: i32 = 0;


// TODO: Update With AI generated Response
const NICE_RESPONSES: [&'static str;6] = ["Thank you!", "I really appreciate that!",
"You're such a good neighbor!", "You're a life saver", "Thanks! I'll see you later.", "Have a good day!"];

const MEAN_RESPONSES: [&'static str;6] = ["Why would you say that to me?", "You're a crazy person!!",
"I will literally call the police.", "Do you want to fight?!?!???!", "You're the worst neighbor EVER!", "You don't want to take it there!"];

const NICE_GREETINGS: [&'static str;6] = ["Hello!", "How are you?", "I hope your day is going good so far!", 
"How is your day going?", "Long time, no see! How are you?", "How's it going?"];

const MEAN_GREETINGS: [&'static str;6] = ["What is WRONG with you?", "Don't smile at me! You KNOW what you did.", "I can not stand you!", 
"You're actually the worst neighbor ever!", "Why do you act like this?", "You're ruining my day!!"];
struct Word(String, i8);

// Spawn all entities to be used in the conversation part of the game
pub fn setup_conversation(
	mut commands: Commands,
	mut clear_color: ResMut<ClearColor>, 
	asset_server: Res<AssetServer>,
){
    unsafe {
        CUR_TURN = START_TURN;
        println!("Current Turn: {}", CUR_TURN);
    }
    clear_color.0 = Color::DARK_GREEN;
    let user_text_style = TextStyle {
		font: asset_server.load("Fonts/SourceSansPro-Regular.ttf"),
        font_size: 40.0,
        color: Color::WHITE
    };
    let enemy_text_style = TextStyle {
		font: asset_server.load("Fonts/SourceSansPro-Regular.ttf"),
        font_size: 60.0,
        color: Color::BLACK
    };

    commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("hero.png"),
		transform: Transform::from_xyz(-500., -225., 2.),
		sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Hero);

	commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("CathyRobinson.png"),
		transform: Transform::from_xyz(500., 200., 2.),
		sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Enemy);

	let box_size = Vec2::new(700.0, 200.0);
    let box_position = Vec2::new(-45.0, -250.0);
    let box_position_two = Vec2::new(45.0, 175.0);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba{red: 0.0, green: 0.0, blue: 0.0, alpha: 0.5},
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position.extend(0.5)),
        ..default()
    }).insert(DialogueBox);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba{red: 255.0, green: 255.0, blue: 255.0, alpha: 0.5},
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position_two.extend(0.0)),
        ..default()
    }).insert(DialogueBox);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Excuse me neighbor, can I borrow some sugar?", enemy_text_style),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position_two.x - box_size.x / 2.0,
            box_position_two.y + box_size.y / 2.0,
            1.0,
        ),
        ..default()
    }).insert(DialogueBox)
    .insert(EnemyDialogue);
    
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Press enter to start", user_text_style),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x - box_size.x / 2.0,
            box_position.y + box_size.y / 2.0,
            1.0,
        ),
        ..default()
    }).insert(DialogueBox)
    .insert(UserInput);
	//info!("Setting Up: GameState: Conversation");
}

// Despawns every entity used in the conversation state that is not also in fight or credits
pub fn clear_conversation(
    mut commands: Commands,
    mut hero: Query<Entity, With<Hero>>,
	mut enemy: Query<Entity, With<Enemy>>,
    dialogue: Query<Entity, With<DialogueBox>>,

) {
    for entity in dialogue.iter() {
        commands.entity(entity).despawn();
    }
    let hero_eid = hero.single_mut();
	let enemy_eid = enemy.single_mut();
    commands.entity(hero_eid).despawn();
	commands.entity(enemy_eid).despawn();
}

// This takes the user's input and then prints every character onto the window using a text box
pub fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
	mut dialogue: Query<&mut Text, With<UserInput>>,
    mut ev_writer: EventWriter<ConvInputEvent>
) {
	let mut dialogue_text = dialogue.single_mut();

	for ev in char_evr.iter() {

		if keys.just_pressed(KeyCode::Return) {
            let entered_string = string.to_string();
            ev_writer.send(ConvInputEvent(entered_string));
			string.clear();	
            dialogue_text.sections[0].value = "".to_string();
		} else
		if keys.just_pressed(KeyCode::Back) {
			string.pop();
			dialogue_text.sections[0].value = string.to_string();
		} else {
			string.push(ev.char); 
			dialogue_text.sections[0].value = string.to_string();
		}
	}
}
// update turn, and returns enemy response to user, handles if final turn
pub fn handle_player_response(
    mut ev_reader: EventReader<ConvInputEvent>,
    mut enemy_dialogue: Query<&mut Text, With<EnemyDialogue>>,
    mut loss_writer: EventWriter<ConvLossEvent>
)
{
    let mut enem_dlg = enemy_dialogue.single_mut();
    for _input in ev_reader.iter() {
        unsafe {
            if CUR_TURN <= MAX_TURNS {
                CUR_TURN = CUR_TURN + 1;
                println!("Current Turn: {}", CUR_TURN);
            }
            else{ // TODO: CASE REACHED FINAL TURN -- NEEDS TO BE HANDLED
                println!("OUT OF RESPONSES: CONV PHASE OVER");
                loss_writer.send(ConvLossEvent());
            }

            let enemy_resp = MEAN_RESPONSES[CUR_TURN as usize];
            println!("Current Turn: {}", CUR_TURN);
            enem_dlg.sections[0].value = enemy_resp.to_string();
        }
        
        
    }
   
}
// Processes the input that the user gives
// For now, just a few key phrases are checked to be contained in the user's response
// This will be where the AI part is implemented
pub fn process_input(
    mut ev_reader: EventReader<ConvInputEvent>,
    mut loss_writer: EventWriter<ConvLossEvent>,
    mut win_writer: EventWriter<ConvWinEvent>
) {
    let mut words = Vec::new();
    let mut score = 0;
    words.push(Word("awesome".to_string(), 10));
    words.push(Word("very".to_string(), 10));
    words.push(Word("yes".to_string(), 10));
    words.push(Word("yeah".to_string(), 10));
    words.push(Word("no".to_string(), -10));
    words.push(Word("not".to_string(), -10));
    for input in ev_reader.iter() {
        let mut string = input.0.to_string();
        string.make_ascii_lowercase();
        string = string.trim_end().to_string();
        for word in string.split_whitespace(){
            for check in words.iter() {
                if &check.0 == word {
                    println!("FOUND A WORD");
                    score = score + &check.1;
                }
            }
        }
        println!("Score: {}", score);

        /*
        if score < 0 {
            loss_writer.send(ConvLossEvent());
        } else {
            win_writer.send(ConvWinEvent());
        }
        */
    }
}