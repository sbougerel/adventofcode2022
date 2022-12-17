use climb::common::*;

fn main() {
    let mut lines = Vec::<Vec<u8>>::new();
    for line in std::io::stdin().lines() {
        lines.push(line.unwrap().bytes().collect());
    }

    if lines.len() == 0 {
        return;
    }

    let mut heightmap = HeightMap::zeros((lines.len(), lines[0].len()));
    let mut starts = Vec::<Pos>::new();
    let mut end: Pos = [0, 0];

    for (row, bytes) in lines.iter().enumerate() {
        for (col, byte) in bytes.iter().enumerate() {
            match byte {
                b'S' | b'a' => {
                    let start = [row, col];
                    heightmap[start] = 0;
                    starts.push(start);
                }
                b'E' => {
                    end = [row, col];
                    heightmap[end] = (b'z' - b'a') as usize;
                }
                b'b'..=b'z' => {
                    heightmap[[row, col]] = (byte - b'a') as usize;
                }
                _ => todo!(),
            }
        }
    }

    // for i in 0..heightmap.nrows() {
    //     for j in 0..heightmap.ncols() {
    //         let h = heightmap[[i, j]];
    //         print!("{h:3}");
    //     }
    //     println!();
    // }

    match shortest_path(heightmap, starts, end) {
        Some(value) => println!("Shortest path: {value}"),
        None => println!("No path to goal"),
    }
}
