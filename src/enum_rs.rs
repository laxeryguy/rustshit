enum Direction {
    North,
    South,
    East, 
    West
}

fn describe(d: Direction) -> &'static str {
    match d {
        Direction::North => "Идём на север",
        Direction::South => "Идём на юг",
        Direction::East => "Идём на восток",
        Direction::West => "Идём на запад",
        _ => "gg"
    }
}

fn main() {
    println!("{}", describe(Direction::East));
}