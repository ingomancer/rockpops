use macroquad::{miniquad::window::screen_size, prelude::*};

fn initial_state() -> (Circle, Rect, Circle, i32, bool) {
    let player = Circle::new(50.0, 90.0, 20.0);
    let paper = Rect {
        x: 200.0,
        y: 200.0,
        w: 35.0,
        h: 35.0,
    };
    let scissor = Circle::new(300.0, 300.0, 25.0);
    let score = 0;
    (player, paper, scissor, score, false)
}

#[macroquad::main("rockpops")]
async fn main() {
    let (mut player, mut paper, mut scissor, mut score, mut game_started) = initial_state();
    let player_speed = 4.0;
    let paper_speed = 2.0;

    loop {
        let (screen_width, screen_height) = screen_size();
        clear_background(BLACK);
        draw_text(
            "You are a rock. Use WASD or arrows to move.",
            10.0,
            10.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Crush scissors but don't get caught by papers!",
            10.0,
            30.0,
            20.0,
            WHITE,
        );

        if game_started {
            draw_text(&format!("Score: {score}"), 10.0, 50.0, 20.0, WHITE);

            player = move_player(player, player_speed);
            player = Circle {
                x: player.x.clamp(0.0 + player.r, screen_width - player.r),
                y: player.y.clamp(0.0 + player.r, screen_height - player.r),
                r: player.r,
            };

            paper = follow_player(paper, player, paper_speed);

            if player.overlaps(&scissor) {
                scissor.x = rand::gen_range(0.0 + scissor.r, screen_width - scissor.r);
                scissor.y = rand::gen_range(0.0 + scissor.r, screen_height - scissor.r);
                score += 1;
            } else {
                scissor.x += rand::gen_range(-1.0, 1.0);
                scissor.y += rand::gen_range(-1.0, 1.0);
            }

            if player.overlaps_rect(&paper) {
                (player, paper, scissor, score, game_started) = initial_state();
            }
        } else {
            draw_text("Press space to start", 10.0, 50.0, 20.0, WHITE);
            if is_key_down(KeyCode::Space) {
                game_started = true;
            }
        }
        draw_hexagon(scissor.x, scissor.y, scissor.r, 2.0, false, RED, GREEN);
        draw_circle(player.x, player.y, player.r, WHITE);
        draw_rectangle(paper.x, paper.y, paper.w, paper.h, RED);

        next_frame().await;
    }
}

fn move_player(player: Circle, player_speed: f32) -> Circle {
    let mut move_y = 0.0;
    let mut move_x = 0.0;
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        move_y -= player_speed;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        move_y += player_speed;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        move_x -= player_speed;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        move_x += player_speed;
    }
    let distance = move_x.hypot(move_y);
    if distance > 0.0 {
        move_x = (move_x / distance) * player_speed;
        move_y = (move_y / distance) * player_speed;
    }
    Circle {
        x: player.x + move_x,
        y: player.y + move_y,
        r: player.r,
    }
}

fn follow_player(paper: Rect, player: Circle, speed: f32) -> Rect {
    let dx = player.x - paper.x;
    let dy = player.y - paper.y;
    let distance = dx.hypot(dy);
    if distance > 0.0 {
        let move_x = (dx / distance) * speed;
        let move_y = (dy / distance) * speed;
        Rect {
            x: paper.x + move_x,
            y: paper.y + move_y,
            w: paper.w,
            h: paper.h,
        }
    } else {
        paper
    }
}
