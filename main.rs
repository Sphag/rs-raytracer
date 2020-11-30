use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter, SeekFrom};
use std::io::prelude::*;
use std::vec::Vec;

use glm::*;

#[derive(Default, Debug)]
struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Vec<UVec3>>,
}

fn load_ppm (path: &Path) -> Result<Image, std::io::Error> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(3))?;

    let buffer = BufReader::new(&file);

    let mut img = Image::default();
    img.pixels.push(Vec::default());

    let mut row_idx = 0;
    let mut col_idx = 0;
    for line in buffer.lines() {
        let line = line?;
        let parsed_line : Vec<usize> = line
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if parsed_line.len() == 1 {
            continue;
        } else if parsed_line.len() == 2 {
            img.width = parsed_line[0];
            img.height = parsed_line[1];
        } else {
            assert_eq!(parsed_line.len(), 3);
            if row_idx == img.width {
                img.pixels.push(Vec::<UVec3>::default());
                col_idx += 1;
                row_idx = 0;
            }
            img.pixels[col_idx].push(UVec3::new(
                parsed_line[0] as u32,
                parsed_line[1] as u32,
                parsed_line[2] as u32,
            ));
            row_idx += 1;
        }
    }

    img.pixels.reverse();

    Ok(img)
}

fn write_ppm (path: &Path, img: &Image) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    let mut buffer = BufWriter::new(file);

    writeln!(buffer, "P3")?;
    writeln!(buffer, "{} {}", img.width, img.height)?;
    writeln!(buffer, "255")?;

    for j in (0usize..img.height).rev() {
        for i in 0usize..img.width {
            let color = img.pixels[j][i];
            writeln!(buffer, "{} {} {}", color.x, color.y, color.z)?;
        }
    }

    Ok(())
}

fn main() {
    println!("Hello, World!");
}