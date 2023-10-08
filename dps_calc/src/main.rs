use std::io;
use rand::Rng;
enum Game {
    Canon,
    Dota,
}
fn main() {
    let mut calculation_results_sum = 0.0;
    let mut number_of_calculations = 1;
    let mut calculation = Calculation { ..Default::default() };
    let mut modifications = Modifications { ..Default::default() };
    let mut attacker = Attacker { ..Default::default() };
    let mut target = Target { ..Default::default() };
    let game_calculated: Game;

    println!("What are we playing today?");

    match input_line().trim() {
        "Canon"|"canon"|"normal"|"default" => game_calculated = Game::Canon,
        "dota 2"|"dota2"|"dota_2"|"dota" => game_calculated = Game::Dota,
        _ => game_calculated = Game::Canon,
    }

    println!("Would you like to customize the settings? y/n");
    match input_line().trim() {
        "Y"|"y" => {
            configure(&mut number_of_calculations, &mut calculation, &mut modifications, &mut attacker, &mut target, &game_calculated);
        },
        _ => println!("Set to default.")
    }
 
    for _ in 0..number_of_calculations {
        match game_calculated {
            Game::Canon => {
                let calculation_result: i32 = Calculation::calculate_canon(&calculation, &attacker, &target, &modifications);
                println!("[canon] total damage: {}", calculation_result);
                calculation_results_sum += calculation_result as f32;
            },
            Game::Dota => {
                let calculation_result: f32 = Calculation::calculate_dota(&calculation, &attacker, &target, &modifications);
                println!("[dota] total damage: {:0.03}", calculation_result);
                calculation_results_sum += calculation_result;
            },
        }
    }

    let average_damage: f32 = calculation_results_sum / number_of_calculations as f32;
    match game_calculated {
        Game::Canon => println!("[canon] average damage over all calculations was: {}", average_damage),
        Game::Dota => println!("[dota] average damage over all calculations was: {:0.02}", average_damage),
    }
    
}

fn configure(number_of_calculations: &mut i32, calculation: &mut Calculation, modifications: &mut Modifications, attacker: &mut Attacker, target: &mut Target, game_calculated: &Game) {
    let mut input_buf: String;
    loop {
        println!("1 - calculation configurations");
        println!("2 - ingame modifications");
        println!("3 - attacker configurations");
        println!("4 - target configurations");
        println!("exit - free cats");
        input_buf = input_line();

        match input_buf.trim() {
            "1" => configure_calculation(number_of_calculations, calculation),
            "2" => configure_modifications(modifications),
            "3" => configure_attacker(attacker, game_calculated),
            "4" => configure_target(target, game_calculated),
            "exit" => println!("meow"),
            _ => println!("you misspelled \"exit\", you idiot"),
        }

        if input_buf.trim() == "exit" { break; }
    }
}

fn configure_target(target: &mut Target, game_calculated: &Game) {
    println!("Armour value:");
    let armour_input: i32;
    let input_buf = input_line().trim().parse().unwrap();
    match input_buf {
        -99999..=99999 => { println!("Set armour to {input_buf}."); armour_input = input_buf;},
        _ => { println!("Set armour to default."); armour_input = 20; },
    }
    Target::set_armour(target, &armour_input, game_calculated);
    println!("Evasion value:");
    let evasion_input: f32;
    let input_buf = input_line().trim().parse().unwrap();
    match input_buf {
        -99999.0..=99999.0 => { println!("Set evasion to {input_buf}."); evasion_input = input_buf;},
        _ => { println!("Set evasion to default."); evasion_input = 0.25; },
    }
    Target::set_evasion(target, &evasion_input, game_calculated);
}

fn configure_attacker(attacker: &mut Attacker, game_calculated: &Game) {
    println!("Attack damage value (default: 10 [canon] / 210 [dota])");
    let attack_input: i32;
    let input_buf = input_line().trim().parse().unwrap();
    match input_buf {
        -99999..=99999 => { println!("Set armour to {input_buf}."); attack_input = input_buf;},
        _ => { println!("Set armour to default."); attack_input = 20; },
    }
    Attacker::set_attack_damage(attacker, &attack_input, game_calculated);
    println!("Attack speed value (default: 1 [canon] / 100 [dota])");
    let attack_speed_input: f32;
    let input_buf = input_line().trim().parse().unwrap();
    match input_buf {
        -99999.0..=99999.0 => { println!("Set attack speed to {input_buf}."); attack_speed_input = input_buf;},
        _ => { println!("Set attack speed to default."); attack_speed_input = 0.25; },
    }
    Attacker::set_attack_speed(attacker, &attack_speed_input, game_calculated);
}

fn configure_modifications(modifications: &mut Modifications) {
    println!("Cuteness value (default: real)");
    let cute_input: bool;
    let input_buf = input_line();
    match input_buf.trim() {
        "cute"|"real"|"True"|"true"|"Yes"|"yes"|"Y"|"y"|"+"|"1" => { println!("Set cute to {0}.", input_buf.trim()); cute_input = true;},
        "uncute" => { println!("wtf"); cute_input = false;},
        _ => { println!("Set cute to real."); cute_input = true; },
    }
    Modifications::set_cute(modifications, &cute_input);
}

fn configure_calculation(number_of_calculations: &mut i32, calculation: &mut Calculation) {
    println!("Number of calculations (default: 4)");
    let input_buf = input_line().trim().parse().unwrap();
    match input_buf {
        1..=1000 => { println!("Set number of calculations to {input_buf}."); *number_of_calculations = input_buf;},
        _ => { println!("Set number of calculations to default."); *number_of_calculations = 4; },
    }
    println!("Time is seconds (default: 10)");
    let time_input: u32;

    let input_buf = input_line().trim().parse().unwrap();
    match input_buf {
        1..=86400 => { println!("Set time to {input_buf}."); time_input = input_buf;},
        _ => { println!("Set time to default."); time_input = 10; },
    }
    Calculation::set_time(calculation, time_input);
    println!("Ticks per second (default: 60)");
    let ticks_input: u8;
    let input_buf = input_line().trim().parse().unwrap();
    match input_buf {
        1..=244 => { println!("Set ticks to {input_buf}."); ticks_input = input_buf;},
        _ => { println!("Set ticks to default."); ticks_input = 60; },
    }
    Calculation::set_ticks(calculation, ticks_input);
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

/* not needed for now */
// fn input_values() -> Vec<u32> {
//     let values: Vec<u32> = input_line()
//     .split_whitespace()
//     .map(|q| q.parse().unwrap())
//     .collect();
//     values
// }

struct Attacker {
    canon_atk: i32,
    canon_atk_speed: u32,
    canon_min_atk: i32,
    canon_max_atk: i32,
    dota_atkspeed_base: f32,
    dota_atkspeed_value: f32,
    dota_damage_value: f32,
    dota_critical_hit_chance: f32,
    dota_critical_hit_multiplier: f32,
    dota_armour_reduction: u32,
}

impl Default for Attacker {
    fn default() -> Self {
        Self {
        canon_atk: 10,
        canon_atk_speed: 1,
        canon_min_atk: 9,
        canon_max_atk: 12,
        dota_atkspeed_base: 1.7,
        dota_atkspeed_value: 200.0,
        dota_damage_value: 210.0,
        dota_critical_hit_chance: 0.3,
        dota_critical_hit_multiplier: 2.25,
        dota_armour_reduction: 6,
        }
    }
}

impl Attacker {
    pub fn set_attack_damage(&mut self, attack_damage: &i32, game: &Game) {
        match game {
            Game::Canon => self.canon_atk = *attack_damage,
            Game::Dota => self.dota_damage_value = *attack_damage as f32
        }
    }
    pub fn set_attack_speed(&mut self, attack_speed: &f32, game: &Game) {
        match game {
            Game::Canon => self.canon_atk_speed = *attack_speed as u32,
            Game::Dota => self.dota_atkspeed_value = *attack_speed
        }
    }
}

struct Target {
    canon_defense_value: i32,
    canon_evasion_chance: u8,
    dota_armour_value: i32,
    dota_evasion_chance: f32,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            canon_defense_value: 2,
            canon_evasion_chance: 10,
            dota_armour_value: 20,
            dota_evasion_chance: 0.25,
        }
    }
}

impl Target {
    pub fn set_armour(&mut self, armour: &i32, game: &Game) {
        match game {
            Game::Canon => self.canon_defense_value = *armour,
            Game::Dota => self.dota_armour_value = *armour
        }
    }
    pub fn set_evasion(&mut self, evasion: &f32, game: &Game) {
        match game {
            Game::Canon => self.canon_evasion_chance = (*evasion * 100.) as u8,
            Game::Dota => self.dota_evasion_chance = *evasion
        }
    }
}

struct Modifications {
    cute: bool,
    canon_randomize_atk_damage: bool,
}

impl Default for Modifications {
    fn default() -> Self {
        Self {
            cute: true,
            canon_randomize_atk_damage: true,
        }
    }
}

impl Modifications {
    pub fn set_cute(&mut self, cute: &bool) {
        self.cute = *cute;
    }
}

struct Calculation {
    time: u32,
    ticks: u8,
}

impl Default for Calculation {
    fn default() -> Self {
        Self {
            time: 10,
            ticks: 60,
        }
    }
}

impl Calculation {
    pub fn set_time(&mut self, time:u32) { self.time = time; }
    pub fn set_ticks(&mut self, ticks:u8) { self.ticks = ticks; }
    pub fn calculate_canon(&self, attacker: &Attacker, target: &Target, other: &Modifications) -> i32 {
        let ticks_total = self.ticks as u32 * self.time;
        let mut rng = rand::thread_rng();
        let mut damage_sum = 0;
        let mut attack_damage: i32;
        let mut damage_recieved: i32;

        for tick in 1..=ticks_total {

            // set attack damage
            if other.canon_randomize_atk_damage == true {
                attack_damage = rng.gen_range(attacker.canon_min_atk..=attacker.canon_max_atk);
            } 
            else { attack_damage = attacker.canon_atk; }

            // modifications
            if other.cute == true { attack_damage += 1; }

            // damage block
            damage_recieved = attack_damage - target.canon_defense_value;

            // no negative damage wtf!!
            if damage_recieved <= 0 { damage_recieved = 0; }

            if tick % (attacker.canon_atk_speed * self.ticks as u32) == 0 { damage_sum += damage_recieved; }
        }

        damage_sum
    }

    pub fn calculate_dota(&self, attacker: &Attacker, target: &Target, other: &Modifications) -> f32 {
        let ticks_total = self.ticks as u32 * self.time;
        let armour_factor = 0.06;
        let armour_value = target.dota_armour_value - attacker.dota_armour_reduction as i32;
        let mut damage_sum = 0.0;

        for tick in 1..=ticks_total {
            let mut dyn_atkspeed_value = attacker.dota_atkspeed_value;     

            // modifications
            if other.cute == true { dyn_atkspeed_value *= 3.75; }

            // attack speed
            if dyn_atkspeed_value >= 700.0 { dyn_atkspeed_value = 700.0; }
            let dota_raw_attack_time = attacker.dota_atkspeed_base / (dyn_atkspeed_value/100.0);
            let ticks_per_attack = (dota_raw_attack_time * self.ticks as f32) as u32;

            // add hit
            if tick % ticks_per_attack == 0 {
                // roll miss
                let missed_hit = Self::dota_roll_missed_hit(target);
                if !missed_hit {
                    // damage reduction
                    let damage_reduction_multiplayer = 1.0-armour_factor*armour_value as f32/(1.0 + armour_factor*armour_value.abs() as f32);

                    // critical damage
                    let critical_damage_multiplier = Self::dota_roll_critical_hit(attacker);

                    // damage recieved
                    let damage_recieved = attacker.dota_damage_value*damage_reduction_multiplayer*critical_damage_multiplier;

                    damage_sum += damage_recieved; 
                } else {
                    damage_sum += 0.;
                }
            }
        }
        // blood thorn?
        if other.cute == true { damage_sum *= 1.3; }
        damage_sum
    }
    pub fn dota_roll_critical_hit(attacker: &Attacker) -> f32 {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let critical_damage_multiplier;
        if roll <= attacker.dota_critical_hit_chance {
            critical_damage_multiplier = attacker.dota_critical_hit_multiplier;
        } else {
            critical_damage_multiplier = 1.;
        }
        critical_damage_multiplier
    }
    pub fn dota_roll_missed_hit(target: &Target) -> bool {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let miss: bool;
        if roll <= target.dota_evasion_chance {
            miss = true;
        } else {
            miss = false;
        }
        miss
    } 
}


