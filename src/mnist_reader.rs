/// Borrowed from https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/

use std::fs;
use std::fs::File;
use std::io::Read;

pub struct Image {
    pub data: Vec<f32>,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .chunks(28)
                .map(|row| row
                    .iter()
                    .map(|pixel| format!("{:3}", pixel))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

pub fn get_data() -> Vec<Image> {
    let data = get_file_as_byte_vec(&"../mnist/train-images-idx3-ubyte".to_string());
    data
        .into_iter()
        .skip(13)
        .map(|x| x as f32)
        .collect::<Vec<f32>>()
        .chunks(28*28)
        .map(|x| Image { data: x.to_vec(), })
        .collect::<Vec<Image>>()
}
