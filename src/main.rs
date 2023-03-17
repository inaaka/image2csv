use csv::Writer;
use image::{DynamicImage, GenericImageView, Rgb};
use std::error::Error;
use std::io::{stdin, stdout, Write};

fn main() {
    let version: &str = "dev_1.0";
    let date: &str = "2023/03/17";

    println!("画像処理プログラム");
    println!("version : {}", version);
    println!("date : {}", date);
    println!("---");

    let input_file: String = input_str("Input image file path");
    let input_image: DynamicImage = open_image(input_file);
    let (width, height) = input_image.dimensions();

    let normalize: Vec<Vec<f32>> = min_max_scaling_image8c(input_image);

    let output_file: String = input_str("Output image file path");
    match write_csv(output_file, normalize, width, height) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write csv: {}", e);
            std::process::exit(1);
        }
    }

    println!("See you...");
}

fn input_str(msg: &str) -> String {
    print!("[INPUT ]{} : ", msg);
    stdout().flush().unwrap();

    let mut input: String = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to read line: {}", e);
            std::process::exit(1);
        }
    }

    input.trim().to_string()
}

fn open_image(input_file: String) -> DynamicImage {
    let img = image::open(input_file);
    match img {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to open image: {}", e);
            std::process::exit(1);
        }
    }

    img.unwrap()
}

fn min_max_scaling_8c(pixel: Rgb<u8>) -> Vec<f32> {
    let max: u8 = 255;
    let min: u8 = 0;
    let diff: f32 = (max - min) as f32;

    let red = (pixel.0[0] - min) as f32 / diff;
    let green = (pixel.0[1] - min) as f32 / diff;
    let blue = (pixel.0[2] - min) as f32 / diff;

    let rgb: Vec<f32> = vec![red, green, blue];
    rgb
}

fn min_max_scaling_image8c(image: DynamicImage) -> Vec<Vec<f32>> {
    let (width, height) = image.dimensions();

    let mut vv: Vec<Vec<f32>> = Vec::new();

    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            let rgb = Rgb([pixel.0[0], pixel.0[1], pixel.0[2]]);

            let v: Vec<f32> = min_max_scaling_8c(rgb);
            vv.push(v);
        }
    }

    vv
}

fn write_csv(
    output_file: String,
    vv: Vec<Vec<f32>>,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn Error>> {
    let mut red_data: Vec<Vec<f32>> = Vec::new();

    for y in 0..width {
        let mut r: Vec<f32> = Vec::new();
        for x in 0..height {
            r.push(vv[(x * height + y) as usize][0]);
        }
        red_data.push(r);
    }

    let mut writer = Writer::from_path(output_file)?;

    for red_v in red_data {
        let v_str: Vec<String> = red_v.iter().map(|f| f.to_string()).collect();
        let record = v_str.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        writer.write_record(&record)?;
    }

    writer.flush()?;

    Ok(())
}
