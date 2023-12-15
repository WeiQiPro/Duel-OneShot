// main.rs
use macroquad::prelude::*;
use std::time::Instant;

mod character;
mod utils;
use utils::{Direction, Vector2, RectEntity};
use character::{Animation, Player};


fn controller(delta: f32, player: &mut Player) {
    let velocity = player.v * delta;
    let mut direction_vector = Vector2::new(0.0, 0.0);

    if !player.is_attacking {
        let right_pressed = is_key_down(KeyCode::Right);
        let left_pressed = is_key_down(KeyCode::Left);
        let up_pressed = is_key_down(KeyCode::Up);
        let down_pressed = is_key_down(KeyCode::Down);

        if right_pressed {
            direction_vector.x += 1.0;
        }
        if left_pressed {
            direction_vector.x -= 1.0;
        }
        if up_pressed {
            direction_vector.y -= 1.0;
        }
        if down_pressed {
            direction_vector.y += 1.0;
        }

        let normalized_vector = direction_vector.normalize();
        player.x += normalized_vector.x * velocity;
        player.y += normalized_vector.y * velocity;

        player.is_moving = normalized_vector.x != 0.0 || normalized_vector.y != 0.0;

        // Set the direction based on the key combination
        if right_pressed && up_pressed {
            player.walk_animation.direction = Direction::NorthEast as i32;
        } else if right_pressed && down_pressed {
            player.walk_animation.direction = Direction::SouthEast as i32;
        } else if left_pressed && up_pressed {
            player.walk_animation.direction = Direction::NorthWest as i32;
        } else if left_pressed && down_pressed {
            player.walk_animation.direction = Direction::SouthWest as i32;
        } else {
            // Set direction for non-diagonal movement
            player.walk_animation.direction =
                match (normalized_vector.x as i32, normalized_vector.y as i32) {
                    (0, -1) => Direction::North as i32,
                    (0, 1) => Direction::South as i32,
                    (1, 0) => Direction::East as i32,
                    (-1, 0) => Direction::West as i32,
                    _ => player.walk_animation.direction,
                };
        }

        if is_key_down(KeyCode::Space) {
            player.is_attacking = true;
            player.attack_animation.current_frame = 0;
            player.attack_animation.direction = player.walk_animation.direction;
            player.is_moving = false;
        }
    }

    if player.is_attacking {
        if player.attack_animation.current_frame >= player.attack_animation.frames - 1 {
            player.is_attacking = false;
        } else {
            player.attack_animation.update();
        }
    } else if player.is_moving {
        player.walk_animation.update();
    }
}

fn check_attack_collision(player: &Player, targets: &mut Vec<&mut RectEntity>) {
    let sweep_angle = 90.0; // Angular width of the attack arc in degrees
                            // Get the player's facing angle and normalize it to 0° - 360°
    let player_facing_angle = get_angle_from_direction(player.attack_animation.direction);

    let player_center_x = player.x + player.attack_radius;
    let player_center_y = player.y + player.attack_radius;

    let lowerbounds: f32 = player_facing_angle - sweep_angle / 2.0;
    let upperbounds: f32 = player_facing_angle + sweep_angle / 2.0;

    for target in targets.iter_mut() {
        let target_center_x = target.x + target.w / 2.0;
        let target_center_y = target.y + target.h / 2.0;

        let mut angle_to_target = (target_center_y - player_center_y)
            .atan2(target_center_x - player_center_x)
            .to_degrees(); // Added & to pass a reference

        if angle_to_target < 0.0 {
            angle_to_target += 360.0;
        }

        if angle_to_target >= 315.0 && angle_to_target <= 360.0 {
            if angle_to_target >= normalize_angle(lowerbounds)
                && angle_to_target <= upperbounds + 360.0
            {
                target.color = RED;
            } else {
                target.color = GREEN;
            }
        } else {
            if angle_to_target >= lowerbounds && angle_to_target <= upperbounds {
                target.color = RED;
            } else {
                target.color = GREEN;
            }
        }
    }
}

fn normalize_angle(angle: f32) -> f32 {
    let sweep = 90.0;
    let mut normalized_angle = angle % (360.0 + (sweep / 2.0));
    if normalized_angle < (0.0 - (sweep / 2.0)) {
        normalized_angle += 360.0;
    }
    normalized_angle
}

fn get_angle_from_direction(direction_int: i32) -> f32 {
    // Create a directional vector for each segment
    let direction_x: f32 = match direction_int {
        0 => 1.0,  // East
        1 => 0.0,  // North
        2 => 1.0,  // North East
        3 => -1.0, // North West
        4 => 0.0,  // South
        5 => 1.0,  // South East
        6 => -1.0, // South West
        7 => -1.0, // West
        _ => 0.0,
    };
    let direction_y: f32 = match direction_int {
        0 => 0.0,  // East
        1 => -1.0, // North
        2 => -1.0, // North East
        3 => -1.0, // North West
        4 => 1.0,  // South
        5 => 1.0,  // South East
        6 => 1.0,  // South West
        7 => 0.0,  // West
        _ => 0.0,
    };
    let mut angle = (direction_y).atan2(direction_x).to_degrees();

    if angle < 0.0 {
        angle += 360.0;
    }

    return angle;
}

fn within_range_of_player<'a>(
    player: &Player,
    entities: &'a mut [RectEntity],
) -> Vec<&'a mut RectEntity> {
    let attack_range = 256.0 / 2.0;
    let player_center_x = player.x + attack_range;
    let player_center_y = player.y + attack_range;
    let mut targets = Vec::new();

    for entity in entities.iter_mut() {
        let entity_center_x = entity.x + entity.w / 2.0;
        let entity_center_y = entity.y + entity.h / 2.0;

        let distance = ((player_center_x - entity_center_x).powi(2)
            + (player_center_y - entity_center_y).powi(2))
        .sqrt();

        if distance <= attack_range + entity.w / 2.0 {
            targets.push(entity);
        }
    }

    targets
}

fn _point_on_circle(center_x: f32, center_y: f32, radius: f32, angle: f32) -> (f32, f32) {
    let x = center_x + radius * angle.cos();
    let y = center_y + radius * angle.sin();
    (x, y)
}

fn _debug_visual(delta: f32) {

    let fps = if delta > 0.0 { 1.0 / delta } else { 0.0 };
    draw_text(&fps.to_string(), 20.0, 20.0, 20.0, BLACK);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Fighter".to_string(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut last_frame_time = Instant::now();
    let mut game_over = false;

    let center_x = 640.0 - 25.0; //sub half of entity width
    let center_y = 360.0 - 25.0; //sub half of entity width
    let offset_distance = 300.0;
    let mut entities = vec![
        RectEntity {
            x: center_x,
            y: center_y - offset_distance,
            w: 50.0,
            h: 50.0,
            color: GREEN,
        }, // North
        RectEntity {
            x: center_x,
            y: center_y + offset_distance,
            w: 50.0,
            h: 50.0,
            color: GREEN,
        }, // South
        RectEntity {
            x: center_x + offset_distance,
            y: center_y,
            w: 50.0,
            h: 50.0,
            color: GREEN,
        }, // East
        RectEntity {
            x: center_x - offset_distance,
            y: center_y,
            w: 50.0,
            h: 50.0,
            color: GREEN,
        }, // West
        RectEntity {
            x: center_x + offset_distance / 1.414,
            y: center_y - offset_distance / 1.414,
            w: 50.0,
            h: 50.0,
            color: GREEN,
        }, // NorthEast
        RectEntity {
            x: center_x - offset_distance / 1.414,
            y: center_y - offset_distance / 1.414,
            w: 50.0,
            h: 50.0,
            color: GREEN,
        }, // NorthWest
        RectEntity {
            x: center_x + offset_distance / 1.414,
            y: center_y + offset_distance / 1.414,
            w: 50.0,
            h: 50.0,
            color: GREEN,
        }, // SouthEast
        RectEntity {
            x: center_x - offset_distance / 1.414,
            y: center_y + offset_distance / 1.414,
            w: 50.0,
            h: 50.0,
            color: GREEN,
        }, // SouthWest
    ];

    let walk_texture = load_texture("src/character/Walk.png").await.unwrap();
    let walk_animation = Animation::new(walk_texture, 24, 0.02, 0);
    let attack_texture = load_texture("src/character/MeleeAttack.png").await.unwrap();
    let attack_animation = Animation::new(attack_texture, 24, 0.04, 0);

    let mut player = Player {
        x: 640.0 - 256.0 / 2.0,
        y: 320.0 - 256.0 / 2.0,
        v: 500.0,
        walk_animation,
        attack_animation,
        is_moving: false,
        is_attacking: false,
        attack_radius: 153.0,
        hitbox: 60.0
    };

    while !game_over {
       
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time);
        last_frame_time = now;
        let delta_seconds = delta_time.as_secs_f32();

        if is_key_down(KeyCode::Escape) {
            game_over = true;
        }

        clear_background(WHITE);
        controller(delta_seconds, &mut player);
        
        _debug_visual(delta_seconds);

        let mut targets = within_range_of_player(&player, &mut entities);

        if player.is_attacking {
            if player.attack_animation.current_frame == 7 {
                check_attack_collision(&player, &mut targets);
            }
            for entity in &mut entities {
                draw_circle(entity.x + 25.0, entity.y + 25.0, 3.0, BLACK);
                draw_rectangle(entity.x, entity.y, entity.w, entity.h, entity.color)
            }
            player.attack_animation.draw(player.x, player.y);
        } else {
            for entity in &mut entities {
                draw_circle(entity.x + 25.0, entity.y + 25.0, 3.0, BLACK);
                draw_rectangle(entity.x, entity.y, entity.w, entity.h, entity.color)
            }
            player.walk_animation.draw(player.x, player.y);
        }

        next_frame().await;
    }
}
