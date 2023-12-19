use std::{fmt::format, io, process::exit};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

// #[derive(Default)]
#[derive(Debug)]
struct Creature {
    id: i32,
    _color: i32,
    _ctype: i32,
    discovered: bool,
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Creature {
    fn init(id: i32, color: i32, ctype: i32) -> Creature {
        Creature {
            id,
            _color: color,
            _ctype: ctype,
            discovered: false,
            x: 0,
            y: 0,
            vx: 0,
            vy: 0,
        }
    }
}

#[derive(Debug)]
struct Board {
    _width: i32,
    _height: i32,
    creature_count: i32,
    creatures: Vec<Creature>,
}

impl Board {
    fn new(input_line: String) -> Self {
        let mut b = Board {
            _width: 10000,
            _height: 10000,
            creature_count: parse_input!(input_line, i32),
            creatures: Vec::new(),
        };

        for _i in 0..b.creature_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            // eprintln!("{}", input_line);
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let creature = Creature::init(
                parse_input!(inputs[0], i32),
                parse_input!(inputs[1], i32),
                parse_input!(inputs[2], i32),
            );
            b.creatures.push(creature);
        }
        return b;
    }

    fn update_creature(&mut self, id: i32, x: i32, y: i32, vx: i32, vy: i32) {
        for c in self.creatures.iter_mut() {
            if c.id == id {
                c.x = x;
                c.y = y;
                c.vx = vx;
                c.vy = vy;
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Drone {
    _drone_id: i32,
    _drone_x: i32,
    _drone_y: i32,
    _emergency: i32,
    _battery: i32,
}

impl Drone {
    fn new() -> Drone {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        Drone {
            _drone_id: parse_input!(inputs[0], i32),
            _drone_x: parse_input!(inputs[1], i32),
            _drone_y: parse_input!(inputs[2], i32),
            _emergency: parse_input!(inputs[3], i32),
            _battery: parse_input!(inputs[4], i32),
        }
    }
}
#[derive(Debug)]
struct Scan {
    _drone_id: i32,
    _creature_id: i32,
}

impl Scan {
    fn new(d_id: i32, c_id: i32) -> Scan {
        Scan {
            _drone_id: d_id,
            _creature_id: c_id,
        }
    }
}

#[derive(Debug)]
struct Blip {
    _drone_id: i32,
    _creature_id: i32,
    _radar: String,
}

impl Blip {
    fn new(d_id: i32, c_id: i32, radar: String) -> Blip {
        Blip {
            _drone_id: d_id,
            _creature_id: c_id,
            _radar: radar,
        }
    }
}

#[derive(Debug)]
struct Ia {
    board: Board,
    my_score: i32,
    foe_score: i32,
    my_scan_count: i32,
    my_scanned_creatures_ids: Vec<i32>,
    foe_scan_count: i32,
    foe_scanned_creatures_ids: Vec<i32>,
    my_drone_cnt: i32,
    my_drones: Vec<Drone>,
    foe_drone_cnt: i32,
    foe_drones: Vec<Drone>,
    drone_scan_count: i32,
    drone_scans: Vec<Scan>,
    visible_creature_cnt: i32,
    radar_blip_cnt: i32,
    blips: Vec<Blip>,
}

impl Ia {
    fn init(board: Board) -> Ia {
        Ia {
            board: board,
            my_score: 0,
            foe_score: 0,
            my_scan_count: 0,
            my_scanned_creatures_ids: Vec::new(),
            foe_scan_count: 0,
            foe_scanned_creatures_ids: Vec::new(),
            my_drone_cnt: 0,
            my_drones: Vec::new(),
            foe_drone_cnt: 0,
            foe_drones: Vec::new(),
            drone_scan_count: 0,
            drone_scans: Vec::new(),
            visible_creature_cnt: 0,
            radar_blip_cnt: 0,
            blips: Vec::new(),
        }
    }

    fn dist_between_drone_and_creature(&self, drone_id: usize, c: &Creature) -> i32 {
        let dx = self.my_drones[drone_id]._drone_x - c.x;
        let dy = self.my_drones[drone_id]._drone_y - c.y;
        return f64::sqrt((dx * dx + dy * dy) as f64) as i32;
    }

    fn update(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.my_score = parse_input!(input_line, i32);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.foe_score = parse_input!(input_line, i32);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.my_scan_count = parse_input!(input_line, i32);
        self.my_scanned_creatures_ids.clear();
        for _i in 0..self.my_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            self.my_scanned_creatures_ids
                .push(parse_input!(input_line, i32));
            for c in self.board.creatures.iter_mut() {
                if c.id == self.my_scanned_creatures_ids.last().unwrap().clone() {
                    c.discovered = true;
                    break;
                }
            }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.foe_scan_count = parse_input!(input_line, i32);
        self.foe_scanned_creatures_ids.clear();
        for _i in 0..self.foe_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            self.foe_scanned_creatures_ids
                .push(parse_input!(input_line, i32));
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.my_drone_cnt = parse_input!(input_line, i32);
        self.my_drones.clear();
        for _i in 0..self.my_drone_cnt as usize {
            self.my_drones.push(Drone::new());
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.foe_drone_cnt = parse_input!(input_line, i32);
        self.foe_drones.clear();
        for _i in 0..self.foe_drone_cnt as usize {
            self.foe_drones.push(Drone::new());
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.drone_scan_count = parse_input!(input_line, i32);
        self.drone_scans.clear();
        for _i in 0..self.drone_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            // eprintln!("{}", input_line);
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            self.drone_scans.push(Scan::new(
                parse_input!(inputs[0], i32),
                parse_input!(inputs[1], i32),
            ));
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.visible_creature_cnt = parse_input!(input_line, i32);
        for _i in 0..self.visible_creature_cnt as usize {
            // Todo: Check if we need to know when it's visible
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            // eprintln!("{}", input_line);
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            self.board.update_creature(
                parse_input!(inputs[0], i32),
                parse_input!(inputs[1], i32),
                parse_input!(inputs[2], i32),
                parse_input!(inputs[3], i32),
                parse_input!(inputs[4], i32),
            );
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // eprintln!("{}", input_line);
        self.radar_blip_cnt = parse_input!(input_line, i32);
        self.blips.clear();
        for _i in 0..self.radar_blip_cnt as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            // eprintln!("{}", input_line);
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            self.blips.push(Blip::new(
                parse_input!(inputs[0], i32),
                parse_input!(inputs[1], i32),
                inputs[2].trim().to_string(),
            ));
        }
    }

    fn decide_move(&mut self, drone_id: usize) -> String {
        let (mut move_to_x, mut move_to_y) = (0, 0);
        let mut closest_move = 100000;

        for c in self.board.creatures.iter() {
            if !c.discovered {
                let dist = self.dist_between_drone_and_creature(drone_id, c);
                if dist < closest_move {
                    closest_move = dist;
                    move_to_x = c.x;
                    move_to_y = c.y;
                }
                eprintln!("closest move to {:#?} is {}", c, closest_move)
            }
        }
        return format!("MOVE {} {}", move_to_x, move_to_y);
    }

    fn decide_scan(&mut self, drone_id: usize) -> String {
        let mut id = 0;
        for c in self.board.creatures.iter() {
            if c.discovered {
                continue;
            }
            let dist = self.dist_between_drone_and_creature(drone_id, c);
            if dist <= 2000 && dist > 800 {
                id = 1;
                eprintln!(
                    "Scan to get creature {:#?} with drone {:#?}",
                    c, self.my_drones[drone_id]
                );
                break;
            }
        }
        return format!("{}", id);
    }

    fn play_drone(&mut self, i: usize) {
        eprintln!("Playing drone {} - {:#?}", i, self.my_drones[i]);
        println!("{} {}", self.decide_move(i), self.decide_scan(i));
    }
}

/**
 * Score points by scanning valuable fish faster than your opponent.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    // eprintln!("{}", input_line);
    let mut ia = Ia::init(Board::new(input_line));

    // game loop
    loop {
        ia.update();
        eprintln!("Ia update done : \n{:#?}", ia);

        for i in 0..ia.my_drone_cnt as usize {
            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");
            ia.play_drone(i);
        }
    }
}
