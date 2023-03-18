use csv::Writer;
use image::{DynamicImage, GenericImageView, GrayImage, Rgb, RgbImage};
use std::error::Error;
use std::io::{stdin, stdout, Write};

fn main() {
    let version: &str = "dev_1.0";
    let date: &str = "2023/03/17";

    println!("画像処理プログラム");
    println!("version : {}", version);
    println!("date : {}", date);
    println!("---");

    // 画像のインプット
    let input_file: String = input_str("Input image file path");
    let input_image: DynamicImage = open_image(input_file);

    // 画像の白黒化（ITU-R Recommendation BT.709）
    // 0.2126 * R + 0.7152 * G + 0.0722 * B
    let gray_image: GrayImage = input_image.into_luma8();

    // 正規化
    let normalize: Vec<Vec<f32>> = min_max_scaling_image8g(gray_image);

    // 画像の出力
    let output_file: String = input_str("Output image file path");
    match write_csv_8g(output_file, normalize) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write csv: {}", e);
            std::process::exit(1);
        }
    }

    // ﾉｼ
    println!("See you...");
}

// 文字列の入力
fn input_str(msg: &str) -> String {
    // 状態表示
    print!("[INPUT ]{} : ", msg);
    stdout().flush().unwrap();

    // 入力
    let mut input: String = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to read line: {}", e);
            std::process::exit(1);
        }
    }

    // 戻り値
    input.trim().to_string()
}

// 画像を開く
fn open_image(input_file: String) -> DynamicImage {
    // 画像を開く
    let img = image::open(input_file);
    match img {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to open image: {}", e);
            std::process::exit(1);
        }
    }

    // 戻り値
    img.unwrap()
}

// 8bit3chカラーピクセルの正規化
fn min_max_scaling_8c(pixel: Rgb<u8>) -> Vec<f32> {
    // 8bitの最大・最小値，差
    let max: u8 = 255;
    let min: u8 = 0;
    let diff: f32 = (max - min) as f32;

    // 正規化
    let red = (pixel.0[0] - min) as f32 / diff;
    let green = (pixel.0[1] - min) as f32 / diff;
    let blue = (pixel.0[2] - min) as f32 / diff;

    // 戻り値
    let rgb: Vec<f32> = vec![red, green, blue];
    rgb
}

// 8bit白黒ピクセルの正規化
fn min_max_scaling_8g(pixel: u8) -> f32 {
    // 8bitの最大・最小値，差
    let max: u8 = 255;
    let min: u8 = 0;
    let diff: f32 = (max - min) as f32;

    // 正規化
    let gray = (pixel - min) as f32 / diff;

    // 戻り値
    gray
}

// 8bit3chカラー画像の正規化
fn min_max_scaling_image8c(image: RgbImage) -> Vec<Vec<Vec<f32>>> {
    // 画像サイズの取得
    let (width, height) = image.dimensions();

    // 各ピクセルの正規化
    let mut vvv: Vec<Vec<Vec<f32>>> = Vec::new();
    for y in 0..height {
        let mut vv: Vec<Vec<f32>> = Vec::new();
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let rgb = Rgb([pixel.0[0], pixel.0[1], pixel.0[2]]);

            let v: Vec<f32> = min_max_scaling_8c(rgb);
            vv.push(v);
        }
        vvv.push(vv);
    }

    // 戻り値
    vvv
}

// 8bit白黒画像の正規化
fn min_max_scaling_image8g(image: GrayImage) -> Vec<Vec<f32>> {
    // 画像サイズの取得
    let (width, height) = image.dimensions();

    // 各ピクセルの正規化
    let mut vv: Vec<Vec<f32>> = Vec::new();

    for y in 0..height {
        let mut v: Vec<f32> = Vec::new();
        for x in 0..width {
            let pixel: u8 = image.get_pixel(x, y)[0];

            let f: f32 = min_max_scaling_8g(pixel);
            v.push(f);
        }
        vv.push(v);
    }

    // 戻り値
    vv
}

// 8bit白黒画像のCSV出力
fn write_csv_8g(output_file: String, vv: Vec<Vec<f32>>) -> Result<(), Box<dyn Error>> {
    // f32データを文字列に変換して書き込み
    let mut writer = Writer::from_path(output_file)?;
    for v in vv {
        let v_str: Vec<String> = v.iter().map(|f| f.to_string()).collect();
        let record = v_str.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        writer.write_record(&record)?;
    }
    writer.flush()?;

    Ok(())
}
