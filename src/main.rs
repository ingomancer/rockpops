use macroquad::{miniquad::window::screen_size, prelude::*};

#[macroquad::main("rockpops")]
async fn main() {
    let mut player = Circle::new(50.0, 50.0, 20.0);
    let mut paper = Rect {
        x: 200.0,
        y: 200.0,
        w: 35.0,
        h: 35.0,
    };
    let mut scissor = Circle::new(300.0, 300.0, 25.0);
    let player_speed = 5.0;
    let paper_speed = 2.0;

    loop {
        let (screen_width, screen_height) = screen_size();
        clear_background(BLACK);
        draw_hexagon(scissor.x, scissor.y, scissor.r, 2.0, false, RED, GREEN);
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
        draw_circle(player.x, player.y, player.r, WHITE);
        draw_rectangle(paper.x, paper.y, paper.w, paper.h, RED);

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            player.y -= player_speed;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            player.y += player_speed;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            player.x -= player_speed;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            player.x += player_speed;
        }

        if player.overlaps_rect(&paper) {
            draw_text("Caught by paper!", 10.0, 50.0, 20.0, WHITE);
        } else {
            draw_text("Safe!", 10.0, 50.0, 20.0, WHITE);
        }

        paper = follow_player(paper, player, paper_speed);

        if player.overlaps(&scissor) {
            scissor.x = rand::gen_range(0.0, screen_width - scissor.r);
            scissor.y = rand::gen_range(0.0, screen_height - scissor.r);
        } else {
            scissor.x += rand::gen_range(-1.0, 1.0);
            scissor.y += rand::gen_range(-1.0, 1.0);
        }

        next_frame().await;
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
