use std::io;
fn main() {
    let mut potion: i32 = 3;
    let mut player_life: f32 = 100.0;
    let mut enemy_life: f32 = 150.0;
    while player_life > 0.0 && enemy_life > 0.0 {
        println!("| Your HP - {} | Boss HP - {} |", player_life, enemy_life);
        println!("| 1) Attack | 2) Defend | 3) Heal |");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading inout");
        let choice = choice.trim();
        if choice == "1" {
            let attack: f32 = receive_player_attack_dmg();
            let damage: f32 = receive_boss_attack_dmg();
            enemy_life -= attack;
            player_life -= damage;
            println!("Your attack deals {} amount of damage.", attack);
            println!("You take {} damage.", damage);
        } else if choice == "2" {
            let defense: f32 = receive_defense_multiplier();
            let damage: f32 = receive_boss_attack_dmg()-defense;
            player_life -= damage;
            println!("Defense activated!");
            println!("You take {} damage.", damage);
        } else if choice == "3" {
            if potion > 0 {
            let health: f32 = receive_health_potion(player_life);
            let damage: f32 = receive_boss_attack_dmg();
            player_life = player_life - damage + health;
            potion -= 1;
            println!("You consume a potion.");
            println!("You take {} damage.", damage);
            } else {
            println!("No more potions");
            }
        } else {
            println!("Wrong input");
        }
    }
    if player_life <= 0.0 {
        println!("You have been defeated");
    } else {
        println!("You have defeated enemy");
    }
}

// attack the main enemy
fn receive_player_attack_dmg() -> f32 {
    let x: f32 = rand::random_range(12.5..20.0);
    return x;
}

//defend yourself
fn receive_defense_multiplier() -> f32 {
    let x: f32 = rand::random_range(2.0..4.0);
    return 1.0/x;
}

// use a potion and improve your life points by 25
fn receive_health_potion(life: f32) -> f32 {
    if life > 0.0 {
        return 25.0;
    } else {
        return 0.0;
    }
}

// attack user
fn receive_boss_attack_dmg() -> f32 {
    let x: f32 = rand::random_range(5.0..25.0);
    return x;
}

