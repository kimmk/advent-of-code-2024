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
    loop {
        if tail <= 0 {
            break;
        }
        if disk[tail] == -1 {
            tail -= 1;
            continue;
        }
        let file_id = disk[tail];
        let mut file_size = 1;
        for file_idx in (0..tail).rev() {
            if disk[file_idx] != file_id {
                break;
            }
            file_size += 1;
        }
        let mut free_pos = 0;
        let mut free_size = 0;
        for head in 0..tail {
            if disk[head] != -1 {
                free_size = 1;
                free_pos = head+1;
            }
            else {
                free_size += 1;
            }
            if free_size > file_size {
                break;
            }
        }
        if free_size >= file_size {
            //println!(
            //    "replacing {}..{} ({:?}) with file id {} of size {}",
            //    free_pos,
            //    free_pos+file_size,
            //    disk[free_pos..free_pos+file_size].to_vec(),
            //    file_id,
            //    file_size,
            //);
            disk.splice(tail-file_size+1..=tail, vec![-1; file_size]);
            disk.splice(free_pos..free_pos+file_size, vec![file_id; file_size]);
            //println!("{:?}\n", disk);
        }
        else {
            //println!("skipping file id {} of size {}",file_id, file_size);
        }
        if tail < file_size {
            break;
        }
        tail -= file_size;
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
