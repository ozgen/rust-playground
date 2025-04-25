struct RedFox {
    enemy: bool,
    life: u8,
}

impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }

    fn set_enemy(&mut self, enemy: bool) {
        self.enemy = enemy;
    }

    fn set_life(&mut self, life: u8) {
        self.life = life;
    }
}

// 'traits' is just like an interface
trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "meow?"
    }
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise())
}

trait Run {
    fn run(&self){
        println!("I am running"); // can set default behaviour like that
    }
}
struct Robot{}
impl Run for Robot{}

fn main() {
    let mut fox = RedFox::new();
    fox.set_enemy(false);
    fox.set_life(100);
    println!("Enemy: {}, Life: {}", fox.enemy, fox.life);
    print_noise(fox);

    let robot = Robot{};
    robot.run();
}
