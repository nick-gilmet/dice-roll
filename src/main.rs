extern crate rand;

use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::io::stdin;

use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let mut dice_map: HashMap<usize, Vec<f64>> = HashMap::new();

    println!("Welcome to the more fair dice roller!\n");
    println!("Enter two numbers:\n  The first is the sizes of the dice, the second\n  is how many of that dice. If no second number is\n  provided then one dice will be rolled.");

    loop {
        let mut s = String::new();
        print!("Dice to roll: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        println!("\n");

        let input = parse_input(&mut s);
        if input.die == 0 || input.num == 0 {
            println!("Invalid input, please roll again...\n");
            continue;
        }

        println!("Result of rolling {} {} sided die:", input.num, input.die);
        handle_dice_roll(&mut rng, &mut dice_map, input);
    }
}

struct Input {
    die: usize,
    num: usize,
}

fn parse_input(s: &mut str) -> Input {
    let split: Vec<&str> = s.trim().split(' ').collect();
    let mut vals: Vec<usize> = vec![];

    for v in split {
        match v.parse::<usize>() {
            Ok(x) => vals.push(x),
            Err(_) => return Input{die: 0, num: 0},
        }
    }
    
    if vals.len() == 1 {
        if vals[0] < 2 {
            return Input{die: 0, num: 0};
        }

        return Input {
            die: vals[0],
            num: 1,
        };
    }
    else if vals.len() == 2 {
        if vals[0] < 2 || vals[1] == 0 {
            return Input{die: 0, num: 0};
        }

        return Input {
            die: vals[0],
            num: vals[1],
        };
    }
    
    return Input{die: 0, num: 0};
}

fn handle_dice_roll(rng: &mut ThreadRng, dice_map: &mut HashMap<usize, Vec<f64>>, roll: Input) {
    maybe_add_dice(dice_map, roll.die);

    // println!("Val: {}", val);

    let vec: &mut Vec<f64> = dice_map.get_mut(&roll.die).unwrap();
    
    for _ in 0..roll.num {
        let mut val: f64 = rng.gen();

        for side in 0..roll.die {
            if val < vec[side] {
                print!("{}, ", side + 1);

                let dist = vec[side] / (roll.die as f64);
                vec[side] = 0f64;
                for i in 0..roll.die {
                    vec[i] += dist;
                }
                break;
            } else {
                val -= vec[side];
            }
        }
    
        let mut count = 0f64;
        for i in 0..roll.die {
            count += vec[i];
        }

        // println!("Count: {}", count);

        *(vec.last_mut().unwrap()) += 1f64 - count;
    }

    // debug
    // print!("\nchances:\n [ ");
    // for v in vec {
    //     print!("{}, ", v);
    // }
    // println!(" ]\n");

    println!("\n");
}

fn maybe_add_dice(dice_map: &mut HashMap<usize, Vec<f64>>, val: usize) {
    if val < 2 {
        return;
    }

    if dice_map.contains_key(&val) {
        return;
    }

    dice_map.insert(val, vec![1f64 / (val as f64); val]);
}