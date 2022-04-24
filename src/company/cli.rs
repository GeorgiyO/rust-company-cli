use std::collections::HashMap;
use std::str::Split;
use lazy_static::lazy_static;

use crate::company::model::{Company, Department};

pub trait Cli<T> {
    fn apply_input(&mut self, input: String) -> String;
}

impl Cli<Company> for Company {
    fn apply_input(&mut self, input: String) -> String {
        let mut words = input.split(" ");
        match words.next() {
            None => "empty input".to_string(),
            Some(x) => self.apply_operation(x, &mut words)
        }
    }
}

lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, &'static str> = [
        ("Add", "Add {person} to {department_name}"),
        ("Remove", "Remove {person} from {department name}"),
        ("Show", "Show from {department name}")
    ].iter().cloned().collect();
}

macro_rules! next_or_return {
    ($from_where: expr, $what_to_return: expr) => {
        match $from_where.next() {
            None => return $what_to_return(),
            Some(x) => x.to_string()
        }
    };
    ($from_where: expr, $what_to_return: expr, $($extra_match: expr),*) => {
        {
            let x = next_or_return!($from_where, $what_to_return);
            match x.as_str() {
                $($extra_match => x,)*
                _ => return $what_to_return()
            }
        }
    }
}

macro_rules! ii {
    ($reason: literal) => (
        || format!("invalid input, expected: {}", COMMANDS.get($reason).unwrap())
    )
}

impl Company {
    fn get_dep(&mut self, key: String) -> &mut Department {
        if !self.contains(&key) {
            self.add(key.clone());
        }
        self.get(&key)
    }

    fn add_cli(&mut self, input: &mut Split<&str>) -> String {
        let ii = ii!("Add");

        let person = next_or_return!(input, ii);
        next_or_return!(input, ii, "to");
        let dep_name = next_or_return!(input, ii);

        let mut dep = self.get_dep(dep_name.clone());
        match dep.contains(&person) {
            true => format!("{} is already in {}", person, dep_name),
            false => {
                dep.add(person.clone());
                format!("{} added to {}", person, dep_name)
            }
        }
    }

    fn remove_cli(&mut self, input: &mut Split<&str>) -> String {
        let ii = ii!("Remove");

        let person = next_or_return!(input, ii);
        next_or_return!(input, ii, "from");
        let dep_name = next_or_return!(input, ii);

        let mut dep = self.get_dep(dep_name.clone());
        match dep.contains(&person) {
            true => {
                dep.remove(&person);
                format!("{} removed from {}", person, dep_name)
            }
            false => format!("{} not found in {}", person, dep_name)
        }
    }

    fn show_cli(&mut self, input: &mut Split<&str>) -> String {
        let ii = ii!("Show");

        next_or_return!(input, ii, "from");
        let dep_name = next_or_return!(input, ii);

        let dep = self.get_dep(dep_name.clone());
        let mut iter = dep.get_all();
        match iter.next() {
            None => format!("department {} is empty", dep_name),
            Some(first) => iter.fold(String::from(first), |a, b| a + " " + b)
        }
    }

    fn apply_operation(&mut self, op: &str, input: &mut Split<&str>) -> String {
        match op {
            "Add" => self.add_cli(input),
            "Remove" => self.remove_cli(input),
            "Show" => self.show_cli(input),
            &_ => {
                let mut v = COMMANDS.values();
                let mut n = || v.next().unwrap();
                format!("invalid command, expected: {} {} {}", n(), n(), n())
            }
        }
    }
}