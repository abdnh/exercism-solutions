use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

use rand::Rng;
use rand::distr::{Distribution, Uniform};

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    robots: Rc<RefCell<HashSet<Robot>>>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Robot {
    name: String,
    factory_robots: Rc<RefCell<HashSet<Robot>>>,
}

impl Hash for Robot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn generate_robot_with_unique_name<R: Rng>(
    robots: Rc<RefCell<HashSet<Robot>>>,
    rng: &mut R,
) -> Robot {
    let letter_dist = Uniform::new_inclusive('A', 'Z').unwrap();
    let num_dist = Uniform::new_inclusive('0', '9').unwrap();
    let mut robot = Robot {
        name: String::new(),
        factory_robots: robots.clone(),
    };

    loop {
        let letters: String = (0..2).map(|_| letter_dist.sample(rng)).collect();
        let nums: String = (0..3).map(|_| num_dist.sample(rng)).collect();
        robot.name = format!("{}{}", letters, nums);
        if !robots.borrow().contains(&robot) {
            break;
        }
    }

    robot
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            robots: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let robot = generate_robot_with_unique_name(self.robots.clone(), rng);
        self.robots.borrow_mut().insert(robot.clone());
        robot
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        let new_robot = generate_robot_with_unique_name(self.factory_robots.clone(), rng);
        self.factory_robots.borrow_mut().remove(self);
        self.name = new_robot.name;
        self.factory_robots.borrow_mut().insert(self.clone());
    }
}
