struct File {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    children: Vec<Item>,
}

enum Item {
    File(File),
    Directory(Directory),
}

fn main() {
    let input = include_str!("example.txt");
}
