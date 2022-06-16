use colour::*;
use std::{
    cmp::Ordering,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let num_to_show = 6;
    //TODO probabilistic showing
    let mut v = read_todos("/home/maxwell/todo.txt")?;
    //v.sort_by(|a, b| b.cmp(a));
    v.sort();
    for todo in v.iter() {
        match todo.prio {
            Priority::Low => {
                magenta_ln!("{}", todo);
            }
            Priority::Medium => {
                yellow_ln!("{}", todo);
            }
            Priority::High => {
                green_ln!("{}", todo);
            }
        }
    }
    //black!("black ");
    //red!("red ");
    //green!("green ");
    //yellow!("yellow ");
    //blue!("blue ");
    //magenta!("magenta ");
    //cyan!("cyan ");
    //white!("white ");
    //dark_red!("dark_red ");
    //dark_green!("dark_green ");
    //dark_yellow!("dark_yellow ");
    //dark_blue!("dark_blue ");
    //dark_magenta!("dark_magenta ");
    //dark_cyan!("dark_cyan ");
    //prnt!("default colour\n\n");
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ret = match (self, other) {
            (Priority::Low, Priority::Low) => Ordering::Equal,
            (Priority::Low, Priority::Medium) => Ordering::Less,
            (Priority::Low, Priority::High) => Ordering::Less,
            (Priority::Medium, Priority::Low) => Ordering::Greater,
            (Priority::Medium, Priority::Medium) => Ordering::Equal,
            (Priority::Medium, Priority::High) => Ordering::Less,
            (Priority::High, Priority::Low) => Ordering::Greater,
            (Priority::High, Priority::Medium) => Ordering::Greater,
            (Priority::High, Priority::High) => Ordering::Equal,
        };
        Some(ret)
    }
}

impl TryFrom<u32> for Priority {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Priority::High),
            2 => Ok(Priority::Medium),
            3 => Ok(Priority::Low),
            _ => Err("invalid number {value}".into()),
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Priority::Low => "High",
            Priority::Medium => "Medium",
            Priority::High => "Low",
        };
        write!(f, "{}", text,)
    }
}

fn read_todos(path: &str) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let buf = BufReader::new(f);

    let mut vec: Vec<Todo> = Vec::new();
    for line in buf.lines() {
        let todo = line?.parse::<Todo>()?;
        vec.push(todo);
    }
    Ok(vec)
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Todo {
    text: String,
    prio: Priority,
}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.prio.partial_cmp(&other.prio)
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl FromStr for Todo {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: Vec<&str> = s.split('#').collect();
        if x.len() > 2 {
            Err("Split is wrong length - should only contain a single # {x}".into())
        } else {
            let text = x[0].to_string();
            let priority: u32;
            if x.len() == 2 {
                priority = u32::from_str(x[1])?;
            } else {
                assert_eq!(x.len(), 1);
                priority = 3;
            }
            Ok(Self {
                text,
                prio: priority.try_into()?,
            })
        }
    }
}
