/// Borrowed from https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/

use std::fs;
use std::fs::File;
use std::io::Read;

pub struct Image {
    pub data: Vec<f32>,
}

pub struct Label {
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

pub fn get_data() -> Vec<(Image, Label)> {
    let image_data = get_file_as_byte_vec(&"../mnist/train-images-idx3-ubyte".to_string());
    let images = image_data
        .into_iter()
        .skip(16)
        .map(|x| x as f32)
        .collect::<Vec<f32>>()
        .chunks(28*28)
        .map(|x| Image { data: x.to_vec(), })
        .collect::<Vec<Image>>();

    let label_data = get_file_as_byte_vec(&"../mnist/train-labels-idx1-ubyte".to_string());
    let labels = label_data
        .into_iter()
        .skip(8)
        .map(|x| Label { data: match x {
            0 => vec!(1., 0., 0., 0., 0., 0., 0., 0., 0., 0.),
            1 => vec!(0., 1., 0., 0., 0., 0., 0., 0., 0., 0.),
            2 => vec!(0., 0., 1., 0., 0., 0., 0., 0., 0., 0.),
            3 => vec!(0., 0., 0., 1., 0., 0., 0., 0., 0., 0.),
            4 => vec!(0., 0., 0., 0., 1., 0., 0., 0., 0., 0.),
            5 => vec!(0., 0., 0., 0., 0., 1., 0., 0., 0., 0.),
            6 => vec!(0., 0., 0., 0., 0., 0., 1., 0., 0., 0.),
            7 => vec!(0., 0., 0., 0., 0., 0., 0., 1., 0., 0.),
            8 => vec!(0., 0., 0., 0., 0., 0., 0., 0., 1., 0.),
            9 => vec!(0., 0., 0., 0., 0., 0., 0., 0., 0., 1.),
            _ => panic!(),
        }})
        .collect::<Vec<Label>>();

    images.into_iter().zip(labels).collect::<Vec<(Image, Label)>>()
}
