use std::io;
use rand::Rng;

fn main() {
    let mut calculation_results_sum = 0.0;
    let mut number_of_calculations = 1;

    let mut settings = Calculation { ..Default::default() };
    //сhoose mode
    let mut attacker = Attacker { ..Default::default() };
    //choose attacker
    let mut target = Target { ..Default::default() };
    //choose reciever
    let mut conditions = Conditions { ..Default::default() };
    //choose other conditions
    let mut calculation_mode: CalculationMode;

    println!("What are we playing today?");

    match input_line().trim() {
        "Canon"|"canon"|"normal"|"default" => calculation_mode = CalculationMode::Canon,
        "dota 2"|"dota2"|"dota_2"|"dota" => calculation_mode = CalculationMode::Dota,
        _ => calculation_mode = CalculationMode::Canon,
    }

    println!("Do you want to configure settings? y/n");
    match input_line().trim() {
        "Y"|"y" => {
            println!("Number of calculations:");
            let input_buf = input_line().trim().parse().unwrap();
            match input_buf {
                1..=1000 => number_of_calculations = input_buf,
                _ => { println!("Set to default."); number_of_calculations = 1; },
            }
        
            println!("Time and ticks:");
            let time_input: u64 = input_line().trim().parse().unwrap();
            let ticks_input: u64 = input_line().trim().parse().unwrap();
            Calculation::set_time(&mut settings, time_input, ticks_input);
        },
        _ => println!("Set to default.")
    }
    
    for _ in 0..number_of_calculations {
        match calculation_mode {
            CalculationMode::Canon => {
                let calculation_result: i64 = Calculation::calculate_canon(&settings, &attacker, &target, &conditions);
                println!("[canon] total damage: {}", calculation_result);
                calculation_results_sum += calculation_result as f64;
            },
            CalculationMode::Dota => {
                let calculation_result: f64 = Calculation::calculate_dota(&settings, &attacker, &target, &conditions);
                println!("[dota] total damage: {:0.03}", calculation_result);
                calculation_results_sum += calculation_result;
            },
        }
    }

    let average_damage: f64 = calculation_results_sum / number_of_calculations as f64;
    match calculation_mode {
        CalculationMode::Canon => println!("[canon] average damage over all calculations was: {}", average_damage),
        CalculationMode::Dota => println!("[dota] average damage over all calculations was: {:0.02}", average_damage),
    }
    
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

/* not needed for now */
// fn input_values() -> Vec<u64> {
//     let values: Vec<u64> = input_line()
//     .split_whitespace()
//     .map(|q| q.parse().unwrap())
//     .collect();
//     values
// }

enum CalculationMode {
    Canon,
    Dota,
}

struct Attacker {
    canon_atk: i64,
    canon_atk_speed: u64,
    canon_min_atk: i64,
    canon_max_atk: i64,
    dota_atkspeed_base: f64,
    dota_atkspeed_value: f64,
    dota_damage_value: f64,
    dota_critical_hit_chance: f64,
    dota_critical_hit_multiplier: f64,
    dota_armour_reduction: u64,
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

struct Target {
    canon_defense: i64,
    dota_armour_value: i64,
    dota_evasion_chance: f64,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            canon_defense: 2,
            dota_armour_value: 20,
            dota_evasion_chance: 0.25,
        }
    }
}

struct Conditions {
    cute: bool,
    canon_randomize_atk_damage: bool,
}

impl Default for Conditions {
    fn default() -> Self {
        Self {
            cute: true,
            canon_randomize_atk_damage: true,
        }
    }
}

struct Calculation {
    time: u64,
    ticks: u64,
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
    #[allow(dead_code)]
    pub fn set_time(&mut self, time:u64, ticks:u64) {
        self.time = time;
        self.ticks = ticks;
    }

    pub fn calculate_canon(&self, character: &Attacker, enemy: &Target, other: &Conditions) -> i64 {
        let ticks_total = self.ticks * self.time;
        let mut rng = rand::thread_rng();
        let mut damage_sum = 0;
        let mut attack_damage: i64;
        let mut damage_recieved: i64;

        for tick in 1..=ticks_total {

            // set attack damage
            if other.canon_randomize_atk_damage == true {
                attack_damage = rng.gen_range(character.canon_min_atk..=character.canon_max_atk);
            } 
            else { attack_damage = character.canon_atk; }

            // modifications
            if other.cute == true { attack_damage += 1; }

            // damage block
            damage_recieved = attack_damage - enemy.canon_defense;

            // no negative damage wtf!!
            if damage_recieved <= 0 { damage_recieved = 0; }

            if tick % (character.canon_atk_speed * self.ticks) == 0 { damage_sum += damage_recieved; }
        }

        damage_sum
    }

    pub fn calculate_dota(&self, character: &Attacker, enemy: &Target, other: &Conditions) -> f64 {
        let ticks_total = self.ticks * self.time;
        let armour_factor = 0.06;
        let armour_value = enemy.dota_armour_value - character.dota_armour_reduction as i64;
        let mut damage_sum = 0.0;

        for tick in 1..=ticks_total {
            let mut dyn_atkspeed_value = character.dota_atkspeed_value;     

            // modifications
            if other.cute == true { dyn_atkspeed_value *= 3.75; }

            // attack speed
            if dyn_atkspeed_value >= 700.0 { dyn_atkspeed_value = 700.0; }
            let dota_raw_attack_time = character.dota_atkspeed_base / (dyn_atkspeed_value/100.0);
            let ticks_per_attack = (dota_raw_attack_time * self.ticks as f64) as u64;

            // add hit
            if tick % ticks_per_attack == 0 {
                // roll miss
                let missed_hit = Self::dota_roll_missed_hit(enemy);
                if !missed_hit {
                    // damage reduction
                    let damage_reduction_multiplayer = 1.0-armour_factor*armour_value as f64/(1.0 + armour_factor*armour_value.abs() as f64);

                    // critical damage
                    let critical_damage_multiplier = Self::dota_roll_critical_hit(character);

                    // damage recieved
                    let damage_recieved = character.dota_damage_value*damage_reduction_multiplayer*critical_damage_multiplier;

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
    pub fn dota_roll_critical_hit(attacker: &Attacker) -> f64 {
        let mut rng = rand::thread_rng();
        let roll: f64 = rng.gen();
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
        let roll: f64 = rng.gen();
        let miss: bool;
        if roll <= target.dota_evasion_chance {
            miss = true;
        } else {
            miss = false;
        }
        miss
    } 
}


