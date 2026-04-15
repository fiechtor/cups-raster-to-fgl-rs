use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use clap::Parser;

use image::ImageBuffer;

const HEADER_SIZE: usize = 1796;

#[derive(Parser, Debug)]
#[command(name = "cups-raster-to-fgl-rs")]
#[command(about = "Converts CUPS Raster v3 format to FGL printer language", long_about = None)]
struct Args {
    #[arg(long, default_value = "false")]
    preview: bool,

    #[arg(long, default_value = "true")]
    preview_raw: bool,

    input: Option<String>,
}

#[repr(C, align(8))]
#[derive(Debug)]
struct CupsRas3 {
    media_class: [u8; 64],
    media_color: [u8; 64],
    media_type: [u8; 64],
    output_type: [u8; 64],
    advance_distance: u32,
    advance_media: u32,
    collate: u32,
    cut_media: u32,
    duplex: u32,
    hw_resolution_h: u32,
    hw_resolution_v: u32,
    imaging_bounding_l: u32,
    imaging_bounding_b: u32,
    imaging_bounding_r: u32,
    imaging_bounding_t: u32,
    insert_sheet: u32,
    jog: u32,
    leading_edge: u32,
    margins_l: u32,
    margins_b: u32,
    manual_feed: u32,
    media_position: u32,
    media_weight: u32,
    mirror_print: u32,
    negative_print: u32,
    num_copies: u32,
    orientation: u32,
    output_face_up: u32,
    page_size_w: u32,
    page_size_h: u32,
    separations: u32,
    tray_switch: u32,
    tumble: u32,
    cups_width: u32,
    cups_height: u32,
    cups_media_type: u32,
    cups_bits_per_color: u32,
    cups_bits_per_pixel: u32,
    cups_bits_per_line: u32,
    cups_color_order: u32,
    cups_color_space: u32,
    cups_compression: u32,
    cups_row_count: u32,
    cups_row_feed: u32,
    cups_row_step: u32,
    cups_num_colors: u32,
    cups_borderless_scaling_factor: f32,
    cups_page_size_w: f32,
    cups_page_size_h: f32,
    cups_imaging_bbox_l: f32,
    cups_imaging_bbox_b: f32,
    cups_imaging_bbox_r: f32,
    cups_imaging_bbox_t: f32,
    cups_integer1: u32,
    cups_integer2: u32,
    cups_integer3: u32,
    cups_integer4: u32,
    cups_integer5: u32,
    cups_integer6: u32,
    cups_integer7: u32,
    cups_integer8: u32,
    cups_integer9: u32,
    cups_integer10: u32,
    cups_integer11: u32,
    cups_integer12: u32,
    cups_integer13: u32,
    cups_integer14: u32,
    cups_integer15: u32,
    cups_integer16: u32,
    cups_real1: f32,
    cups_real2: f32,
    cups_real3: f32,
    cups_real4: f32,
    cups_real5: f32,
    cups_real6: f32,
    cups_real7: f32,
    cups_real8: f32,
    cups_real9: f32,
    cups_real10: f32,
    cups_real11: f32,
    cups_real12: f32,
    cups_real13: f32,
    cups_real14: f32,
    cups_real15: f32,
    cups_real16: f32,
    cups_string1: [u8; 64],
    cups_string2: [u8; 64],
    cups_string3: [u8; 64],
    cups_string4: [u8; 64],
    cups_string5: [u8; 64],
    cups_string6: [u8; 64],
    cups_string7: [u8; 64],
    cups_string8: [u8; 64],
    cups_string9: [u8; 64],
    cups_string10: [u8; 64],
    cups_string11: [u8; 64],
    cups_string12: [u8; 64],
    cups_string13: [u8; 64],
    cups_string14: [u8; 64],
    cups_string15: [u8; 64],
    cups_string16: [u8; 64],
    cups_marker_type: [u8; 64],
    cups_rendering_intent: [u8; 64],
    cups_page_size_name: [u8; 64],
}

fn read_le_u32(data: &[u8]) -> u32 {
    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
}

fn read_le_f32(data: &[u8]) -> f32 {
    f32::from_le_bytes([data[0], data[1], data[2], data[3]])
}

fn parse_header(data: &[u8]) -> CupsRas3 {
    let mut offset = 0;

    fn read_bytes(data: &[u8], offset: &mut usize, len: usize) -> [u8; 64] {
        let mut result = [0u8; 64];
        let src = &data[*offset..*offset + len.min(64)];
        result[..src.len()].copy_from_slice(src);
        *offset += 64;
        result
    }

    fn read_u32(data: &[u8], offset: &mut usize) -> u32 {
        let val = read_le_u32(&data[*offset..*offset + 4]);
        *offset += 4;
        val
    }

    fn read_f32(data: &[u8], offset: &mut usize) -> f32 {
        let val = read_le_f32(&data[*offset..*offset + 4]);
        *offset += 4;
        val
    }

    let media_class = read_bytes(data, &mut offset, 64);
    let media_color = read_bytes(data, &mut offset, 64);
    let media_type = read_bytes(data, &mut offset, 64);
    let output_type = read_bytes(data, &mut offset, 64);

    let advance_distance = read_u32(data, &mut offset);
    let advance_media = read_u32(data, &mut offset);
    let collate = read_u32(data, &mut offset);
    let cut_media = read_u32(data, &mut offset);
    let duplex = read_u32(data, &mut offset);
    let hw_resolution_h = read_u32(data, &mut offset);
    let hw_resolution_v = read_u32(data, &mut offset);
    let imaging_bounding_l = read_u32(data, &mut offset);
    let imaging_bounding_b = read_u32(data, &mut offset);
    let imaging_bounding_r = read_u32(data, &mut offset);
    let imaging_bounding_t = read_u32(data, &mut offset);
    let insert_sheet = read_u32(data, &mut offset);
    let jog = read_u32(data, &mut offset);
    let leading_edge = read_u32(data, &mut offset);
    let margins_l = read_u32(data, &mut offset);
    let margins_b = read_u32(data, &mut offset);
    let manual_feed = read_u32(data, &mut offset);
    let media_position = read_u32(data, &mut offset);
    let media_weight = read_u32(data, &mut offset);
    let mirror_print = read_u32(data, &mut offset);
    let negative_print = read_u32(data, &mut offset);
    let num_copies = read_u32(data, &mut offset);
    let orientation = read_u32(data, &mut offset);
    let output_face_up = read_u32(data, &mut offset);
    let page_size_w = read_u32(data, &mut offset);
    let page_size_h = read_u32(data, &mut offset);
    let separations = read_u32(data, &mut offset);
    let tray_switch = read_u32(data, &mut offset);
    let tumble = read_u32(data, &mut offset);
    let cups_width = read_u32(data, &mut offset);
    let cups_height = read_u32(data, &mut offset);
    let cups_media_type = read_u32(data, &mut offset);
    let cups_bits_per_color = read_u32(data, &mut offset);
    let cups_bits_per_pixel = read_u32(data, &mut offset);
    let cups_bits_per_line = read_u32(data, &mut offset);
    let cups_color_order = read_u32(data, &mut offset);
    let cups_color_space = read_u32(data, &mut offset);
    let cups_compression = read_u32(data, &mut offset);
    let cups_row_count = read_u32(data, &mut offset);
    let cups_row_feed = read_u32(data, &mut offset);
    let cups_row_step = read_u32(data, &mut offset);
    let cups_num_colors = read_u32(data, &mut offset);
    let cups_borderless_scaling_factor = read_f32(data, &mut offset);
    let cups_page_size_w = read_f32(data, &mut offset);
    let cups_page_size_h = read_f32(data, &mut offset);
    let cups_imaging_bbox_l = read_f32(data, &mut offset);
    let cups_imaging_bbox_b = read_f32(data, &mut offset);
    let cups_imaging_bbox_r = read_f32(data, &mut offset);
    let cups_imaging_bbox_t = read_f32(data, &mut offset);

    let mut cups_integer = vec![];
    for _ in 0..16 {
        cups_integer.push(read_u32(data, &mut offset));
    }

    let mut cups_real = vec![];
    for _ in 0..16 {
        cups_real.push(read_f32(data, &mut offset));
    }

    let mut cups_string = vec![];
    for _ in 0..16 {
        cups_string.push(read_bytes(data, &mut offset, 64));
    }

    let cups_marker_type = read_bytes(data, &mut offset, 64);
    let cups_rendering_intent = read_bytes(data, &mut offset, 64);
    let cups_page_size_name = read_bytes(data, &mut offset, 64);

    CupsRas3 {
        media_class,
        media_color,
        media_type,
        output_type,
        advance_distance,
        advance_media,
        collate,
        cut_media,
        duplex,
        hw_resolution_h,
        hw_resolution_v,
        imaging_bounding_l,
        imaging_bounding_b,
        imaging_bounding_r,
        imaging_bounding_t,
        insert_sheet,
        jog,
        leading_edge,
        margins_l,
        margins_b,
        manual_feed,
        media_position,
        media_weight,
        mirror_print,
        negative_print,
        num_copies,
        orientation,
        output_face_up,
        page_size_w,
        page_size_h,
        separations,
        tray_switch,
        tumble,
        cups_width,
        cups_height,
        cups_media_type,
        cups_bits_per_color,
        cups_bits_per_pixel,
        cups_bits_per_line,
        cups_color_order,
        cups_color_space,
        cups_compression,
        cups_row_count,
        cups_row_feed,
        cups_row_step,
        cups_num_colors,
        cups_borderless_scaling_factor,
        cups_page_size_w,
        cups_page_size_h,
        cups_imaging_bbox_l,
        cups_imaging_bbox_b,
        cups_imaging_bbox_r,
        cups_imaging_bbox_t,
        cups_integer1: cups_integer[0],
        cups_integer2: cups_integer[1],
        cups_integer3: cups_integer[2],
        cups_integer4: cups_integer[3],
        cups_integer5: cups_integer[4],
        cups_integer6: cups_integer[5],
        cups_integer7: cups_integer[6],
        cups_integer8: cups_integer[7],
        cups_integer9: cups_integer[8],
        cups_integer10: cups_integer[9],
        cups_integer11: cups_integer[10],
        cups_integer12: cups_integer[11],
        cups_integer13: cups_integer[12],
        cups_integer14: cups_integer[13],
        cups_integer15: cups_integer[14],
        cups_integer16: cups_integer[15],
        cups_real1: cups_real[0],
        cups_real2: cups_real[1],
        cups_real3: cups_real[2],
        cups_real4: cups_real[3],
        cups_real5: cups_real[4],
        cups_real6: cups_real[5],
        cups_real7: cups_real[6],
        cups_real8: cups_real[7],
        cups_real9: cups_real[8],
        cups_real10: cups_real[9],
        cups_real11: cups_real[10],
        cups_real12: cups_real[11],
        cups_real13: cups_real[12],
        cups_real14: cups_real[13],
        cups_real15: cups_real[14],
        cups_real16: cups_real[15],
        cups_string1: cups_string[0],
        cups_string2: cups_string[1],
        cups_string3: cups_string[2],
        cups_string4: cups_string[3],
        cups_string5: cups_string[4],
        cups_string6: cups_string[5],
        cups_string7: cups_string[6],
        cups_string8: cups_string[7],
        cups_string9: cups_string[8],
        cups_string10: cups_string[9],
        cups_string11: cups_string[10],
        cups_string12: cups_string[11],
        cups_string13: cups_string[12],
        cups_string14: cups_string[13],
        cups_string15: cups_string[14],
        cups_string16: cups_string[15],
        cups_marker_type,
        cups_rendering_intent,
        cups_page_size_name,
    }
}

fn generate_preview(
    pixels: &[u8],
    width: u32,
    height: u32,
    path: &str,
    raw: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    const SCALE: u32 = 2;
    let scaled_width = width * SCALE;
    let scaled_height = height * SCALE;

    let img = image::GrayImage::from_raw(width, height, pixels.to_vec())
        .ok_or("Failed to create image")?;

    let preview_img: ImageBuffer<image::Luma<u8>, Vec<u8>> = if raw {
        // Scale raw pixels 2x2
        ImageBuffer::from_fn(scaled_width, scaled_height, |x, y| {
            let orig_x = x / SCALE;
            let orig_y = y / SCALE;
            let pixel = img.get_pixel(orig_x, orig_y);
            *pixel
        })
    } else {
        // Scale binary pixels 2x2
        let thresholded: image::GrayImage = image::imageops::grayscale(&img);
        ImageBuffer::from_fn(scaled_width, scaled_height, |x, y| {
            let orig_x = x / SCALE;
            let orig_y = y / SCALE;
            let pixel = thresholded.get_pixel(orig_x, orig_y);
            image::Luma([if pixel[0] < 128 { 0 } else { 255 }])
        })
    };

    preview_img.save(path)?;
    Ok(())
}

fn detect_format(data: &[u8]) -> Result<&'static str, &'static str> {
    if data.len() < 4 {
        return Err("Not enough data");
    }

    let magic = &data[0..4];
    if magic == b"RaS3" || magic == b"3SaR" {
        return Ok("ras3");
    }

    // Check for FGL commands (skip leading whitespace)
    let mut pos = 0;
    while pos < data.len()
        && (data[pos] == b' ' || data[pos] == b'\t' || data[pos] == b'\n' || data[pos] == b'\r')
    {
        pos += 1;
    }

    if pos < data.len() && data[pos] == b'<' {
        return Ok("fgl");
    }

    Err("Unknown format")
}

fn parse_fgl_commands(data: &[u8]) -> Result<(Vec<FglCommand>, Vec<u8>), &'static str> {
    let mut commands = Vec::new();
    let mut raw_bytes = Vec::new();

    // Skip leading whitespace
    let mut pos = 0;
    while pos < data.len()
        && (data[pos] == b' ' || data[pos] == b'\t' || data[pos] == b'\n' || data[pos] == b'\r')
    {
        pos += 1;
    }

    while pos < data.len() {
        // Only parse a command if we're at a '<' that starts a valid command
        if data[pos] != b'<' {
            pos += 1;
            continue;
        }

        // Find the closing '>'
        let mut end = pos + 1;
        while end < data.len() && data[end] != b'>' {
            end += 1;
        }

        if end >= data.len() {
            break;
        }

        let cmd_bytes = &data[pos + 1..end];

        // Check if this is a valid command (RC, G, CB, p, q)
        let cmd_str = std::str::from_utf8(cmd_bytes).ok();
        let is_valid_cmd = match cmd_str {
            Some(s) => {
                s.starts_with("RC") || s.starts_with('G') || s == "CB" || s == "p" || s == "q"
            }
            None => false,
        };

        if !is_valid_cmd {
            // Not a valid command, skip this '<' and continue
            pos += 1;
            continue;
        }

        if cmd_str.unwrap().starts_with("RC") {
            let content = &cmd_str.unwrap()[2..];
            let parts: Vec<&str> = content.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(y), Ok(x)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    commands.push(FglCommand::RC { y, x });
                }
            }
        } else if cmd_str.unwrap().starts_with('G') {
            if let Ok(count) = cmd_str.unwrap()[1..].parse::<usize>() {
                commands.push(FglCommand::G { count });
                // Collect the raw bytes that follow this command
                let bytes_start = end + 1;
                let bytes_end = bytes_start + count;
                if bytes_end <= data.len() {
                    raw_bytes.extend_from_slice(&data[bytes_start..bytes_end]);
                }
                pos = bytes_end;
                continue;
            }
        } else if cmd_str.unwrap() == "CB" || cmd_str.unwrap() == "p" || cmd_str.unwrap() == "q" {
            commands.push(FglCommand::G { count: 0 });
        }

        pos = end + 1;
    }

    Ok((commands, raw_bytes))
}

#[derive(Debug)]
enum FglCommand {
    RC { y: u32, x: u32 },
    G { count: usize },
}

fn render_fgl_preview(
    commands: &[FglCommand],
    raw_bytes: &[u8],
    width: u32,
    height: u32,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Scale factor for preview (2x2 pixels per logical pixel)
    const SCALE: u32 = 2;

    let scaled_width = width * SCALE;
    let scaled_height = height * SCALE;

    let mut pixels = vec![255u8; (scaled_width * scaled_height) as usize];

    let mut current_y = 0;
    let mut current_x = 0;
    let mut data_offset = 0;

    for cmd in commands {
        match cmd {
            FglCommand::RC { y, x } => {
                current_y = *y;
                current_x = *x;
            }
            FglCommand::G { count } => {
                if *count > 0 {
                    for i in 0..*count {
                        if data_offset + i < raw_bytes.len() {
                            let byte = raw_bytes[data_offset + i];

                            // Each byte is 1 column of 8 dots (vertical)
                            // MSB (bit 7) = top dot, LSB (bit 0) = bottom dot
                            for bit in 0..8 {
                                let pixel_y = current_y + bit as u32;
                                let pixel_x = current_x + i as u32;

                                if pixel_x < width && pixel_y < height {
                                    let bit_set = (byte & (1 << (7 - bit))) != 0;
                                    let pixel_val = if bit_set { 0 } else { 255 };

                                    // Scale the pixel 2x2
                                    let scaled_x = pixel_x * SCALE;
                                    let scaled_y = pixel_y * SCALE;

                                    for sy in 0..SCALE {
                                        for sx in 0..SCALE {
                                            let scaled_idx = ((scaled_y + sy) * scaled_width
                                                + (scaled_x + sx))
                                                as usize;
                                            if scaled_idx < pixels.len() {
                                                pixels[scaled_idx] = pixel_val;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    data_offset += count;
                }
            }
        }
    }

    let img =
        ImageBuffer::<image::Luma<u8>, Vec<u8>>::from_raw(scaled_width, scaled_height, pixels)
            .ok_or("Failed to create preview image")?;
    img.save(path)?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    let mut input = Vec::new();
    let input_path = args.input.clone();

    if let Some(path) = input_path {
        File::open(&path)
            .map_err(|e| eprintln!("Failed to open file: {}", e))
            .unwrap()
            .read_to_end(&mut input)
            .unwrap();
    } else {
        io::stdin().read_to_end(&mut input).unwrap();
    }

    if input.len() < 4 {
        eprintln!("Error: Not enough data");
        std::process::exit(1);
    }

    let format = match detect_format(&input) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match format {
        "ras3" => process_ras3(&args, input),
        "fgl" => process_fgl(&args, input),
        _ => unreachable!(),
    }
}

fn process_ras3(args: &Args, input: Vec<u8>) {
    let _is_little_endian = &input[0..4] == b"RaS3";

    let mut offset = 4;
    let mut page_num = 0;

    while offset < input.len() {
        if offset + HEADER_SIZE > input.len() {
            break;
        }

        let header = parse_header(&input[offset..offset + HEADER_SIZE]);
        offset += HEADER_SIZE;

        if header.cups_color_space != 0 || header.cups_num_colors != 1 {
            eprintln!("Error: Invalid color space, only monocolor supported");
            std::process::exit(1);
        }

        let bytes_per_pixel = (header.cups_bits_per_pixel as usize + 7) / 8;
        let row_bytes = (header.cups_width as usize * bytes_per_pixel + 7) / 8;
        let img_size = row_bytes * header.cups_height as usize;

        if offset + img_size > input.len() {
            break;
        }

        let img_data = &input[offset..offset + img_size];
        offset += img_size;

        let mut pixels: Vec<u8> =
            Vec::with_capacity((header.cups_width * header.cups_height) as usize);

        for y in 0..header.cups_height as usize {
            for x in 0..header.cups_width as usize {
                let byte_idx = y * row_bytes + x * bytes_per_pixel;
                let pixel_val = img_data[byte_idx];
                pixels.push(pixel_val);
            }
        }

        let img =
            image::GrayImage::from_raw(header.cups_width as u32, header.cups_height as u32, pixels)
                .expect("Failed to create image");

        let gray_img = img.clone();
        let thresholded: image::GrayImage = image::imageops::grayscale(&gray_img);

        let mut transposed = vec![0u8; (header.cups_width * header.cups_height) as usize];
        for y in 0..header.cups_height as usize {
            for x in 0..header.cups_width as usize {
                let pixel = thresholded.get_pixel(x as u32, y as u32);
                transposed[y * header.cups_width as usize + x] = if pixel[0] < 128 { 1 } else { 0 };
            }
        }

        if args.preview {
            let preview_path = if args.preview_raw {
                generate_preview_filename(&args.input, "raw", page_num)
            } else {
                generate_preview_filename(&args.input, "processed", page_num)
            };

            if let Err(e) = generate_preview(
                &img.into_raw(),
                header.cups_width,
                header.cups_height,
                &preview_path,
                args.preview_raw,
            ) {
                eprintln!("Failed to generate preview: {}", e);
            }
        }

        io::stdout().write_all(b"<CB>").unwrap();

        for yoffset in (0..header.cups_height as usize).step_by(8) {
            let mut row_octet = vec![0u8; header.cups_width as usize];

            for j in 0..8 {
                let row_y = yoffset + j;
                if row_y >= header.cups_height as usize {
                    break;
                }
                for x in 0..header.cups_width as usize {
                    let pixel = transposed[row_y * header.cups_width as usize + x];
                    if pixel == 0 {
                        row_octet[x] |= 1 << (7 - j);
                    }
                }
            }

            if row_octet.iter().any(|&b| b != 0) {
                print!("<RC{},0><G{}>", yoffset, row_octet.len());
                io::stdout().write_all(&row_octet).unwrap();
            }
        }

        if (header.cut_media == 1 || header.cut_media == 2 || header.cut_media == 3)
            && offset >= input.len()
        {
            io::stdout().write_all(b"<p>").unwrap();
        } else if header.cut_media == 4 {
            io::stdout().write_all(b"<p>").unwrap();
        } else {
            io::stdout().write_all(b"<q>").unwrap();
        }

        page_num += 1;
    }
}

fn process_fgl(args: &Args, input: Vec<u8>) {
    let (commands, raw_bytes) = match parse_fgl_commands(&input) {
        Ok((c, b)) => (c, b),
        Err(e) => {
            eprintln!("Failed to parse FGL: {}", e);
            std::process::exit(1);
        }
    };

    let (width, height) = determine_image_dimensions(&commands);

    if args.preview {
        let preview_path = generate_preview_filename(&args.input, "fgl", 0);

        if let Err(e) = render_fgl_preview(&commands, &raw_bytes, width, height, &preview_path) {
            eprintln!("Failed to generate FGL preview: {}", e);
        }
    }

    io::stdout().write_all(&input).unwrap();
}

fn generate_preview_filename(input_path: &Option<String>, suffix: &str, page_num: u32) -> String {
    if let Some(path) = input_path {
        let p = Path::new(path);
        let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("preview");

        if page_num > 0 {
            format!("{}_{}_{}.png", stem, suffix, page_num + 1)
        } else {
            format!("{}_{}.png", stem, suffix)
        }
    } else {
        if page_num > 0 {
            format!("preview_{}_{}.png", suffix, page_num + 1)
        } else {
            format!("preview_{}.png", suffix)
        }
    }
}

fn determine_image_dimensions(commands: &[FglCommand]) -> (u32, u32) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut current_x = 0;
    let mut current_y = 0;

    for cmd in commands {
        match cmd {
            FglCommand::RC { y, x } => {
                current_y = *y;
                current_x = *x;
            }
            FglCommand::G { count } => {
                // Each byte in G command is one column
                let end_x = current_x + *count as u32;
                if end_x > max_x {
                    max_x = end_x;
                }
                let end_y = current_y + 8;
                if end_y > max_y {
                    max_y = end_y;
                }
            }
        }
    }

    (max_x, max_y)
}
