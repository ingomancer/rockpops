use macroquad::{miniquad::window::screen_size, prelude::*};

fn initial_state() -> (
    Vec<Circle>,
    Vec<String>,
    Vec<Texture2D>,
    i32,
    bool,
    bool,
    bool,
) {
    let rock =
        Texture2D::from_file_with_format(include_bytes!("../data/noto-emoji/rock_1faa8.png"), None);
    let paper = Texture2D::from_file_with_format(
        include_bytes!("../data/noto-emoji/roll-of-paper_1f9fb.png"),
        None,
    );
    let scissors = Texture2D::from_file_with_format(
        include_bytes!("../data/noto-emoji/scissors_2702-fe0f.png"),
        None,
    );
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
        vec![
            "rock".to_string(),
            "paper".to_string(),
            "scissors".to_string(),
        ],
        vec![rock, paper, scissors],
        0,
        false,
        false,
        false,
    )
}

fn draw_base(names: &[String], mouse_controls: bool) {
    clear_background(BLACK);
    let control_string = if mouse_controls {
        "It follows the pointer"
    } else {
        "WASD or arrows to move"
    };

    draw_text(
        &format!("You are {}. {}", names[0], control_string),
        10.0,
        10.0,
        20.0,
        WHITE,
    );
    draw_text(
        &format!("Catch {} but don't get caught by {}!", names[2], names[1]),
        10.0,
        30.0,
        20.0,
        WHITE,
    );
}

#[macroquad::main("Rock Paper Shuffle!")]
async fn main() {
    let (
        mut actors,
        mut names,
        mut textures,
        mut score,
        mut game_started,
        mut rotate_enabled,
        mut mouse_controls,
    ) = initial_state();
    let player_speed = 4.0;
    let enemy_speed = 2.0;
    let mut start_time = 0;

    loop {
        let (screen_width, screen_height) = screen_size();
        draw_base(&names, mouse_controls);
        if game_started {
            let gametime = get_time_seconds() - start_time;
            draw_text(
                &format!("Score: {score}, playtime: {gametime}"),
                10.0,
                50.0,
                20.0,
                WHITE,
            );
            if mouse_controls {
                let mouse_pos = mouse_position();
                let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);
                actors[0] = follow_target(actors[0], mouse_pos, player_speed);
            } else {
                actors[0] = move_player(actors[0], player_speed);
            }

            actors[0] = Circle {
                x: actors[0]
                    .x
                    .clamp(0.0 + actors[0].r, screen_width - actors[0].r),
                y: actors[0]
                    .y
                    .clamp(0.0 + actors[0].r, screen_height - actors[0].r),
                r: actors[0].r,
            };

            actors[1] = follow_target(actors[1], actors[0].point(), enemy_speed);

            if actors[0].overlaps(&actors[2]) {
                actors[2].x = rand::gen_range(0.0 + actors[2].r, screen_width - actors[2].r);
                actors[2].y = rand::gen_range(0.0 + actors[2].r, screen_height - actors[2].r);
                score += 1;
            } else {
                actors[2].x += rand::gen_range(-1.0, 1.0);
                actors[2].y += rand::gen_range(-1.0, 1.0);
            }

            if actors[0].overlaps(&actors[1]) {
                (
                    actors,
                    names,
                    textures,
                    score,
                    game_started,
                    rotate_enabled,
                    mouse_controls,
                ) = initial_state();
            }

            countdown(gametime, screen_width, screen_height);

            if rotate_enabled && (gametime) % 10 == 0 {
                let direction = rand::gen_range(1, 3);
                actors.rotate_left(direction);
                names.rotate_left(direction);
                textures.rotate_left(direction);
                rotate_enabled = false;
            }

            if ((gametime) + 1) % 10 == 0 {
                rotate_enabled = true;
            }
            draw_actors(&actors, &textures);
        } else {
            draw_text(
                "Press space to start (or click to start in mouse mode)",
                10.0,
                50.0,
                20.0,
                WHITE,
            );
            if is_key_down(KeyCode::Space) {
                game_started = true;
                start_time = get_time_seconds();
            } else if is_mouse_button_pressed(MouseButton::Left) {
                game_started = true;
                start_time = get_time_seconds();
                mouse_controls = true;
            }
        }

        next_frame().await;
    }
}
#[allow(clippy::cast_possible_truncation)]
fn get_time_seconds() -> i64 {
    let time = get_time();
    time.round() as i64
}

fn countdown(gametime: i64, screen_width: f32, screen_height: f32) {
    let screen_center = Vec2::new(screen_width / 2.0, screen_height / 2.0);

    if (gametime + 2) % 10 == 0 {
        draw_text("Rock...", screen_center.x, screen_center.y, 50.0, WHITE);
    }
    if (gametime + 1) % 10 == 0 {
        draw_text("Paper...", screen_center.x, screen_center.y, 70.0, WHITE);
    }
    if (gametime > 0) && (gametime) % 10 == 0 {
        draw_text("SHUFFLE!", screen_center.x, screen_center.y, 90.0, WHITE);
    }
}

fn draw_actors(actors: &[Circle], textures: &[Texture2D]) {
    let time = get_time();
    let drawparams = DrawTextureParams {
        dest_size: Some(Vec2::new(actors[0].r * 2.0, actors[0].r * 2.0)),
        source: None,
        #[allow(clippy::cast_possible_truncation)]
        rotation: time as f32,
        pivot: None,
        flip_x: false,
        flip_y: false,
    };
    draw_texture_ex(
        &textures[0],
        actors[0].x - actors[0].r,
        actors[0].y - actors[0].r,
        WHITE,
        drawparams.clone(),
    );
    draw_texture_ex(
        &textures[1],
        actors[1].x - actors[1].r,
        actors[1].y - actors[1].r,
        WHITE,
        drawparams.clone(),
    );
    draw_texture_ex(
        &textures[2],
        actors[2].x - actors[2].r,
        actors[2].y - actors[2].r,
        WHITE,
        drawparams,
    );
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

fn follow_target(follower: Circle, target: Vec2, speed: f32) -> Circle {
    let dx = target.x - follower.x;
    let dy = target.y - follower.y;
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
