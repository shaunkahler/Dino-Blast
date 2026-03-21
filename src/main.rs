use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};

const NEON_PINK: Color = Color::new(1.0, 0.0, 0.5, 1.0);
const NEON_CYAN: Color = Color::new(0.0, 0.9, 1.0, 1.0);
const DEEP_PURPLE: Color = Color::new(0.2, 0.0, 0.3, 1.0);
const SUNSET_ORANGE: Color = Color::new(1.0, 0.5, 0.0, 1.0);
const NEON_YELLOW: Color = Color::new(1.0, 1.0, 0.2, 1.0);
const GOOGLE_RED: Color = Color::new(0.86, 0.26, 0.22, 1.0);
const GOOGLE_GREEN: Color = Color::new(0.2, 0.62, 0.32, 1.0);
const GOOGLE_BLUE: Color = Color::new(0.26, 0.52, 0.96, 1.0);

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;
const GROUND_Y: f32 = 100.0;
const GRAVITY: f32 = 1600.0;
const JUMP_FORCE: f32 = -750.0;

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    lifetime: f32,
    is_gmail: bool,
    from_enemy: bool,
}

#[derive(PartialEq)]
enum EnemyType {
    Dino,
    Heli,
}

struct Enemy {
    pos: Vec2,
    speed: f32,
    health: f32,
    color: Color,
    anim_timer: f32,
    enemy_type: EnemyType,
    shoot_timer: f32,
}

struct Player {
    pos: Vec2,
    vel: Vec2,
    speed: f32,
    bullets: Vec<Bullet>,
    shoot_timer: f32,
    score: i32,
    health: f32,
    shake_timer: f32,
    is_grounded: bool,
    invincibility_timer: f32,
    muzzle_flash: f32,
    aim_dir: Vec2,
}

fn draw_gmail_icon(pos: Vec2, size: f32, bold: bool) {
    let thickness = if bold { 4.0 } else { 2.0 };
    draw_rectangle(pos.x, pos.y, size, size * 0.7, WHITE);
    draw_rectangle_lines(pos.x, pos.y, size, size * 0.7, 2.0, GOOGLE_RED);
    draw_line(pos.x, pos.y, pos.x + size / 2.0, pos.y + size / 2.0, thickness, GOOGLE_RED);
    draw_line(pos.x + size, pos.y, pos.x + size / 2.0, pos.y + size / 2.0, thickness, GOOGLE_RED);
    draw_line(pos.x, pos.y, pos.x, pos.y + size * 0.7, thickness, GOOGLE_RED);
    draw_line(pos.x + size, pos.y, pos.x + size, pos.y + size * 0.7, thickness, GOOGLE_RED);
}

fn draw_heli(pos: Vec2, size: f32, color: Color, anim: f32) {
    let rotor_offset = (anim * 30.0).sin() * size * 0.5;
    draw_rectangle(pos.x, pos.y, size * 1.3, size * 0.9, color);
    draw_gmail_icon(pos + vec2(size * 0.15, size * 0.1), size * 1.0, true);
    draw_line(pos.x + size * 0.65 - rotor_offset, pos.y - 12.0, pos.x + size * 0.65 + rotor_offset, pos.y - 12.0, 5.0, WHITE);
    draw_rectangle(pos.x + size * 0.6, pos.y - 12.0, 5.0, 12.0, DARKGRAY);
    draw_rectangle(pos.x - size * 0.5, pos.y + size * 0.25, size * 0.5, size * 0.25, color);
    draw_rectangle(pos.x - size * 0.5, pos.y, 5.0, size * 0.7, color);
}

fn draw_chrome_dino(pos: Vec2, size: f32, color: Color, anim: f32) {
    let leg_offset = (anim * 15.0).sin() * 5.0;
    draw_rectangle(pos.x - 5.0, pos.y + size * 0.5, 10.0, 10.0, color);
    draw_rectangle(pos.x, pos.y, size * 0.8, size, color);
    draw_rectangle(pos.x + size * 0.3, pos.y - size * 0.4, size, size * 0.6, color);
    draw_rectangle(pos.x + size * 0.6, pos.y - size * 0.3, 5.0, 5.0, WHITE);
    draw_rectangle(pos.x + size * 0.7, pos.y + size * 0.3, 8.0, 4.0, color);
    draw_rectangle(pos.x + 5.0, pos.y + size, 8.0, 12.0 + leg_offset, color);
    draw_rectangle(pos.x + size * 0.5, pos.y + size, 8.0, 12.0 - leg_offset, color);
}

fn draw_neon_palm(x: f32, y: f32) {
    draw_rectangle(x - 5.0, y - 120.0, 10.0, 120.0, Color::new(0.3, 0.15, 0.0, 1.0));
    let leaf_color = GOOGLE_GREEN;
    for i in 0..6 {
        let angle = i as f32 * 1.0 - 0.5;
        let end_x = x + angle.cos() * 60.0;
        let end_y = y - 120.0 + angle.sin() * 30.0;
        draw_line(x, y - 120.0, end_x, end_y, 4.0, leaf_color);
        draw_circle(end_x, end_y, 4.0, leaf_color);
    }
}

fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    let cos = angle.cos();
    let sin = angle.sin();
    vec2(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

fn draw_rectangle_rotated(x: f32, y: f32, w: f32, h: f32, angle: f32, color: Color) {
    let pivot = vec2(x, y);
    let p1 = pivot + rotate_vec2(vec2(0.0, -h/2.0), angle);
    let p2 = pivot + rotate_vec2(vec2(w, -h/2.0), angle);
    let p3 = pivot + rotate_vec2(vec2(w, h/2.0), angle);
    let p4 = pivot + rotate_vec2(vec2(0.0, h/2.0), angle);
    
    draw_triangle(p1, p2, p3, color);
    draw_triangle(p1, p3, p4, color);
}

fn draw_miami_guy(pos: Vec2, anim: f32, is_grounded: bool, aim_dir: Vec2) {
    let bob = if is_grounded { (anim * 12.0).sin() * 2.0 } else { 0.0 };
    draw_rectangle(pos.x + 5.0, pos.y + 30.0 + bob, 10.0, 15.0, NEON_PINK);
    draw_rectangle(pos.x + 20.0, pos.y + 30.0 + bob, 10.0, 15.0, NEON_PINK);
    draw_rectangle(pos.x, pos.y + 10.0 + bob, 35.0, 25.0, NEON_PINK);
    draw_rectangle(pos.x + 10.0, pos.y + 10.0 + bob, 15.0, 20.0, NEON_CYAN);
    draw_rectangle(pos.x + 10.0, pos.y - 5.0 + bob, 15.0, 15.0, Color::new(1.0, 0.8, 0.6, 1.0));
    draw_rectangle(pos.x + 15.0, pos.y + bob, 10.0, 4.0, BLACK);
    let gun_center = pos + vec2(20.0, 20.0 + bob);
    let angle = aim_dir.y.atan2(aim_dir.x);
    draw_rectangle_rotated(gun_center.x, gun_center.y, 35.0, 8.0, angle, DARKGRAY);
}

#[macroquad::main("Miami Pixel Google Shooter EXTREME")]
async fn main() {
    let mut player = Player {
        pos: vec2(100.0, 300.0),
        vel: vec2(0.0, 0.0),
        speed: 450.0,
        bullets: Vec::new(),
        shoot_timer: 0.0,
        score: 0,
        health: 100.0,
        shake_timer: 0.0,
        is_grounded: false,
        invincibility_timer: 0.0,
        muzzle_flash: 0.0,
        aim_dir: vec2(1.0, 0.0),
    };

    let mut enemies: Vec<Enemy> = Vec::new();
    let mut parallax_x = 0.0;
    let mut spawn_timer = 0.0;
    let mut game_over = false;
    let mut anim_time = 0.0;

    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();
        anim_time += dt;

        if !game_over {
            player.vel.y += GRAVITY * dt;
            player.pos += player.vel * dt;

            if player.pos.y >= VIRTUAL_HEIGHT - GROUND_Y - 45.0 {
                player.pos.y = VIRTUAL_HEIGHT - GROUND_Y - 45.0;
                player.vel.y = 0.0;
                player.is_grounded = true;
            } else {
                player.is_grounded = false;
            }

            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { player.pos.x -= player.speed * dt; }
            if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { player.pos.x += player.speed * dt; }
            if (is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up)) && player.is_grounded {
                player.vel.y = JUMP_FORCE;
            }

            let mouse_pos = mouse_position();
            let scale = (screen_width() / VIRTUAL_WIDTH).min(screen_height() / VIRTUAL_HEIGHT);
            let offset_x = (screen_width() - VIRTUAL_WIDTH * scale) / 2.0;
            let offset_y = (screen_height() - VIRTUAL_HEIGHT * scale) / 2.0;
            let virtual_mouse = vec2((mouse_pos.0 - offset_x) / scale, (mouse_pos.1 - offset_y) / scale);
            let gun_center = player.pos + vec2(20.0, 20.0);
            
            if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
                player.aim_dir = (virtual_mouse - gun_center).normalize();
            }

            player.shoot_timer -= dt;
            let is_shooting = is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::J) || is_key_down(KeyCode::K);
            if is_shooting {
                if player.shoot_timer <= 0.0 {
                    let is_gmail = rand::gen_range(0, 5) == 0;
                    player.bullets.push(Bullet {
                        pos: gun_center + player.aim_dir * 30.0,
                        vel: player.aim_dir * 1400.0,
                        lifetime: 1.2,
                        is_gmail,
                        from_enemy: false,
                    });
                    player.shoot_timer = 0.08;
                }
                player.shake_timer = 0.1;
            } else {
                player.shake_timer = 0.0;
            }

            spawn_timer -= dt;
            if spawn_timer <= 0.0 {
                let is_heli = rand::gen_range(0, 3) == 0;
                let (etype, ecolor, y_pos) = if is_heli {
                    (EnemyType::Heli, WHITE, rand::gen_range(50.0, 300.0))
                } else {
                    let colors = [GOOGLE_RED, GOOGLE_GREEN, GOOGLE_BLUE, NEON_YELLOW];
                    (EnemyType::Dino, colors[rand::gen_range(0, 4)], VIRTUAL_HEIGHT - GROUND_Y - 40.0)
                };
                enemies.push(Enemy {
                    pos: vec2(VIRTUAL_WIDTH, y_pos),
                    speed: rand::gen_range(200.0, 500.0),
                    health: if is_heli { 5.0 } else { 3.0 },
                    color: ecolor,
                    anim_timer: 0.0,
                    enemy_type: etype,
                    shoot_timer: rand::gen_range(1.0, 3.0),
                });
                spawn_timer = rand::gen_range(0.8, 2.0);
            }

            player.bullets.retain_mut(|b| {
                b.pos += b.vel * dt;
                b.lifetime -= dt;
                if b.from_enemy {
                    let player_rect = Rect::new(player.pos.x, player.pos.y, 40.0, 45.0);
                    if player_rect.contains(b.pos) && player.invincibility_timer <= 0.0 {
                        player.health -= 10.0;
                        player.invincibility_timer = 1.0;
                        b.lifetime = 0.0;
                    }
                }
                b.lifetime > 0.0
            });

            enemies.retain_mut(|e| {
                e.pos.x -= e.speed * dt;
                if e.enemy_type == EnemyType::Heli {
                    e.pos.y += (anim_time * 2.0).sin() * 2.0;
                    e.shoot_timer -= dt;
                    if e.shoot_timer <= 0.0 {
                        let to_player = (player.pos + vec2(20.0, 20.0) - e.pos).normalize();
                        player.bullets.push(Bullet {
                            pos: e.pos,
                            vel: to_player * 400.0,
                            lifetime: 3.0,
                            is_gmail: false,
                            from_enemy: true,
                        });
                        e.shoot_timer = rand::gen_range(2.0, 4.0);
                    }
                }
                e.anim_timer += dt;
                for b in &mut player.bullets {
                    if !b.from_enemy && (b.pos - (e.pos + vec2(20.0, 10.0))).length() < 40.0 {
                        let damage = if b.is_gmail { 3.0 } else { 1.0 };
                        e.health -= damage;
                        b.lifetime = 0.0;
                        player.score += (damage * 10.0) as i32;
                    }
                }
                let player_rect = Rect::new(player.pos.x, player.pos.y, 40.0, 45.0);
                let enemy_rect = Rect::new(e.pos.x, e.pos.y, 40.0, 40.0);
                if player_rect.overlaps(&enemy_rect) && player.invincibility_timer <= 0.0 {
                    player.health -= 20.0;
                    player.invincibility_timer = 1.0;
                }
                if player.health <= 0.0 { game_over = true; }
                e.health > 0.0 && e.pos.x > -200.0
            });

            if player.invincibility_timer > 0.0 { player.invincibility_timer -= dt; }
            parallax_x += 70.0 * dt;
        } else {
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::R) {
                player.pos = vec2(100.0, 300.0);
                player.health = 100.0;
                player.score = 0;
                player.bullets.clear();
                enemies.clear();
                game_over = false;
            }
        }

        // --- DRAW ---
        set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT)));
        
        for i in 0..VIRTUAL_HEIGHT as i32 {
            let t = i as f32 / VIRTUAL_HEIGHT;
            let color = if t < 0.5 {
                let local_t = t * 2.0;
                Color::new(DEEP_PURPLE.r + (NEON_PINK.r - DEEP_PURPLE.r) * local_t, DEEP_PURPLE.g + (NEON_PINK.g - DEEP_PURPLE.g) * local_t, DEEP_PURPLE.b + (NEON_PINK.b - DEEP_PURPLE.b) * local_t, 1.0)
            } else {
                let local_t = (t - 0.5) * 2.0;
                Color::new(NEON_PINK.r + (SUNSET_ORANGE.r - NEON_PINK.r) * local_t, NEON_PINK.g + (SUNSET_ORANGE.g - NEON_PINK.g) * local_t, NEON_PINK.b + (SUNSET_ORANGE.b - NEON_PINK.b) * local_t, 1.0)
            };
            draw_line(0.0, i as f32, VIRTUAL_WIDTH, i as f32, 1.0, color);
        }

        let camera_offset = if player.shake_timer > 0.0 { vec2(rand::gen_range(-6.0, 6.0), rand::gen_range(-6.0, 6.0)) } else { vec2(0.0, 0.0) };

        draw_circle(VIRTUAL_WIDTH * 0.8 + camera_offset.x, VIRTUAL_HEIGHT * 0.2 + camera_offset.y, 80.0, NEON_YELLOW);
        for i in 0..8 {
            let x = (i as f32 * 350.0 - parallax_x) % (VIRTUAL_WIDTH + 350.0);
            draw_neon_palm(x, VIRTUAL_HEIGHT - GROUND_Y);
        }

        draw_rectangle(0.0, VIRTUAL_HEIGHT - GROUND_Y, VIRTUAL_WIDTH, GROUND_Y, NEON_PINK);
        draw_line(0.0, VIRTUAL_HEIGHT - GROUND_Y, VIRTUAL_WIDTH, VIRTUAL_HEIGHT - GROUND_Y, 4.0, NEON_CYAN);

        for b in &player.bullets {
            let color = if b.from_enemy { GOOGLE_RED } else { NEON_YELLOW };
            if b.is_gmail { draw_gmail_icon(b.pos + camera_offset, 22.0, false); }
            else { draw_circle(b.pos.x + camera_offset.x, b.pos.y + camera_offset.y, 5.0, color); }
        }

        for e in &enemies {
            match e.enemy_type {
                EnemyType::Dino => draw_chrome_dino(e.pos + camera_offset, 38.0, e.color, e.anim_timer),
                EnemyType::Heli => draw_heli(e.pos + camera_offset, 40.0, e.color, e.anim_timer),
            }
        }

        if !game_over && (player.invincibility_timer <= 0.0 || (player.invincibility_timer * 12.0) as i32 % 2 == 0) {
            draw_miami_guy(player.pos + camera_offset, anim_time, player.is_grounded, player.aim_dir);
        }

        if game_over {
            draw_rectangle(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.5));
            draw_text("WASTED IN MIAMI", VIRTUAL_WIDTH/2.0 - 180.0, VIRTUAL_HEIGHT/2.0, 50.0, RED);
        }

        draw_rectangle(15.0, 15.0, 260.0, 130.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_text(&format!("G-SCORE: {:06}", player.score), 25.0, 80.0, 32.0, WHITE);
        draw_rectangle(90.0, 102.0, 160.0 * (player.health / 100.0), 18.0, GOOGLE_RED);

        set_default_camera();
        let scale = (screen_width() / VIRTUAL_WIDTH).min(screen_height() / VIRTUAL_HEIGHT);
        let w = VIRTUAL_WIDTH * scale;
        let h = VIRTUAL_HEIGHT * scale;
        let x = (screen_width() - w) / 2.0;
        let y = (screen_height() - h) / 2.0;
        draw_texture_ex(render_target.texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(w, h)), ..Default::default() });

        next_frame().await
    }
}
