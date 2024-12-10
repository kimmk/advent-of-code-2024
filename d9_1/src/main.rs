struct Block {
    id: i32,
    file: bool,
    size: i32,
    free: i32,
}

struct Disk {
    blocks: Vec<Block>,
}

fn build_disk(
    map: &str
) -> Disk {
    let mut disk = Disk {
        blocks: vec![],
    };
    let mut id = 0;
    let mut file = true;
    for size in map.chars().map(|ch| ch.to_digit(10).unwrap() as i32).collect::<Vec<i32>>() {
        if size > 0 {
            disk.blocks.push(Block {
                id: id,
                file: file,
                size: size,
                free: 0,
            });
        }
        if file {
            id += 1;
        }
        file = !file;
    }
    return disk;
}

fn disk_print(
    disk: &Disk,
) -> Vec<i32> {
    let mut disk_vec = vec![];
    for block in &disk.blocks {
        let mut id = -1;
        if block.file {
            id = block.id;
        }
        disk_vec.extend(vec![id; block.size as usize]);
    }
    disk_vec
}

fn defrag(
    disk_vec: &Vec<i32>,
) -> Vec<i32> {
    let mut disk = disk_vec.clone();
    let mut tail = disk.len()-1;
    let mut head = 0;
    loop {
        if head >= tail || head >= disk.len() {
            break;
        }
        if disk[head] != -1 {
            head += 1;
            continue;
        }
        if disk[tail] == -1 {
            tail -= 1;
            continue;
        }
        disk[head] = disk[tail];
        disk[tail] = -1;
        head += 1;
        tail -= 1;
    }
    disk
}

fn disk_checksum(
    disk: &Vec<i32>
) -> u64 {
    let mut cs = 0;
    for (pos, bt) in disk.iter().enumerate() {
        if *bt == -1 {
            continue;
        }
        cs += (pos as u64) * (*bt as u64);
    }
    return cs;
}

fn main() {
    let input = include_str!("input");
    let disk = build_disk(&input[..input.len()-1]);
    let disk_vec = disk_print(&disk);
    println!("{:?}", disk_vec);
    let defrag_disk = defrag(&disk_vec);
    println!("{:?}", defrag_disk);
    println!("checksum: {}", disk_checksum(&defrag_disk));
}
