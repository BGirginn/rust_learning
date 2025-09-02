enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::Up => println!("Yukarı"),
        Direction::Down => println!("Aşağı"),
        Direction::Left => println!("Sola"),
        Direction::Right => println!("Sağa"),
    }
}

fn main() {
    move_player(Direction::Up);
}
