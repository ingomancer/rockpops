use macroquad::{miniquad::window::screen_size, prelude::*};

fn initial_state() -> (Vec<Circle>, i32, bool) {
    (
        vec![
            Circle::new(50.0, 90.0, 20.0),
            Circle {
                x: 200.0,
                y: 200.0,
                r: 20.0,
            },
            Circle::new(300.0, 300.0, 20.0),
        ],
        0,
        false,
    )
}

#[macroquad::main("Rock Paper Shuffle!")]
async fn main() {
    let mut names = [
        "rock".to_string(),
        "paper".to_string(),
        "scissors".to_string(),
    ];
    let mut colors = [WHITE, RED, GREEN];
    let mut rotate_enabled = false;
    let (mut actors, mut score, mut game_started) = initial_state();
    let pspeed = 4.0;
    let papseed = 2.0;
    let mut start_time = get_time_seconds();

    loop {
        let (screen_width, screen_height) = screen_size();
        clear_background(BLACK);
        draw_text(
            &format!("You are {}. Use WASD or arrows to move.", names[0]),
            10.0,
            10.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Crush {} but don't get caught by {}!", names[2], names[1]),
            10.0,
            30.0,
            20.0,
            WHITE,
        );

        if game_started {
            draw_text(
                &format!(
                    "Score: {score}, playtime: {}",
                    get_time_seconds() - start_time
                ),
                10.0,
                50.0,
                20.0,
                WHITE,
            );

            actors[0] = move_player(actors[0], pspeed);
            actors[0] = Circle {
                x: actors[0]
                    .x
                    .clamp(0.0 + actors[0].r, screen_width - actors[0].r),
                y: actors[0]
                    .y
                    .clamp(0.0 + actors[0].r, screen_height - actors[0].r),
                r: actors[0].r,
            };

            actors[1] = follow_player(actors[1], actors[0], papseed);

            if actors[0].overlaps(&actors[2]) {
                actors[2].x = rand::gen_range(0.0 + actors[2].r, screen_width - actors[2].r);
                actors[2].y = rand::gen_range(0.0 + actors[2].r, screen_height - actors[2].r);
                score += 1;
            } else {
                actors[2].x += rand::gen_range(-1.0, 1.0);
                actors[2].y += rand::gen_range(-1.0, 1.0);
            }

            if actors[0].overlaps(&actors[1]) {
                (actors, score, game_started) = initial_state();
            }

            if rotate_enabled && (get_time_seconds() - start_time) % 10 == 0 {
                let direction = rand::gen_range(1, 3);
                actors.rotate_left(direction);
                names.rotate_left(direction);
                colors.rotate_left(direction);
                rotate_enabled = false;
            }

            if ((get_time_seconds() - start_time) + 1) % 10 == 0 {
                rotate_enabled = true;
            }
        } else {
            draw_text("Press space to start", 10.0, 50.0, 20.0, WHITE);
            if is_key_down(KeyCode::Space) {
                game_started = true;
                start_time = get_time_seconds();
            }
        }
        draw_circle(actors[0].x, actors[0].y, actors[0].r, colors[0]);
        draw_circle(actors[1].x, actors[1].y, actors[1].r, colors[1]);
        draw_circle(actors[2].x, actors[2].y, actors[2].r, colors[2]);

        next_frame().await;
    }
}
#[allow(clippy::cast_possible_truncation)]
fn get_time_seconds() -> i64 {
    let time = get_time();
    time.round() as i64
}

fn move_player(player: Circle, pspeed: f32) -> Circle {
    let mut move_y = 0.0;
    let mut move_x = 0.0;
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        move_y -= pspeed;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        move_y += pspeed;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        move_x -= pspeed;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        move_x += pspeed;
    }
    let distance = move_x.hypot(move_y);
    if distance > 0.0 {
        move_x = (move_x / distance) * pspeed;
        move_y = (move_y / distance) * pspeed;
    }
    Circle {
        x: player.x + move_x,
        y: player.y + move_y,
        r: player.r,
    }
}

fn follow_player(follower: Circle, player: Circle, speed: f32) -> Circle {
    let dx = player.x - follower.x;
    let dy = player.y - follower.y;
    let distance = dx.hypot(dy);
    if distance > 0.0 {
        let move_x = (dx / distance) * speed;
        let move_y = (dy / distance) * speed;
        Circle {
            x: follower.x + move_x,
            y: follower.y + move_y,
            r: follower.r,
        }
    } else {
        follower
    }
}
