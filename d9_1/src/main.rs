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
) -> String {
    let mut disk_str = String::new();
    for block in &disk.blocks {
        let mut id = String::from(".");
        if block.file {
            id = block.id.to_string();
        }
        disk_str = format!("{}{}", disk_str, id.repeat(block.size as usize));
    }
    disk_str
}

fn defrag(
    disk_str: String,
) -> String {
    let mut disk = disk_str.as_bytes().to_vec();
    let mut tail = disk.len()-1;
    let mut head = 0;
    loop {
        //println!("{}", String::from_utf8(disk.clone()).unwrap());
        if head >= tail || head >= disk.len() {
            break;
        }
        if disk[head] != b'.' {
            head += 1;
            continue;
        }
        if disk[tail] == b'.' {
            tail -= 1;
            continue;
        }
        disk[head] = disk[tail];
        disk[tail] = b'.';
        head += 1;
        tail -= 1;
    }
    String::from_utf8(disk).unwrap()
}

fn disk_checksum(
    disk_str: &String
) -> u64 {
    let disk = disk_str.as_bytes().to_vec();
    let mut cs = 0;
    for (pos, bt) in disk.iter().enumerate() {
        if *bt == b'.' {
            continue;
        }
        let idx = *bt as u8 - b'0';
        cs += (pos as u64) * (idx as u64);
    }
    return cs;
}

fn main() {
    let input = include_str!("input");
    let disk = build_disk(&input[..input.len()-1]);
    let disk_str = disk_print(&disk);
    println!("{}", disk_str);
    let defrag_disk = defrag(disk_str);
    println!("{}", defrag_disk);
    println!("checksum: {}", disk_checksum(&defrag_disk));
}
