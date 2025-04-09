use macroquad::prelude::*;

#[macroquad::main("rockpops")]
async fn main() {
    let mut player_pos = Vec2::new(50.0, 50.0);
    let paper_pos = Vec2::new(200.0, 200.0);
    let scissor_pos = Vec2::new(300.0, 300.0);
    let player_speed = 5.0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            player_pos.y -= player_speed;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            player_pos.y += player_speed;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            player_pos.x -= player_speed;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            player_pos.x += player_speed;
        }

        draw_circle(player_pos.x, player_pos.y, 20.0, WHITE);
        draw_rectangle(paper_pos.x, paper_pos.y, 50.0, 50.0, RED);
        if player_pos.distance(paper_pos) < 35.0 {
            draw_text("Caught by paper!", 10.0, 50.0, 20.0, WHITE);
        } else {
            draw_text("Safe!", 10.0, 50.0, 20.0, WHITE);
        }

        if player_pos.distance(scissor_pos) < 35.0 {
            draw_text("Crushed scissors!", 10.0, 70.0, 20.0, WHITE);
        } else {
            draw_text("Scissors are safe!", 10.0, 70.0, 20.0, WHITE);
        }
        draw_hexagon(scissor_pos.x, scissor_pos.y, 25.0, 2.0, false, RED, GREEN);
        draw_text(
            "You are a rock. Use WASD or arrows to move. Crush scissors but don't get caught by papers!",
            10.0,
            10.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
