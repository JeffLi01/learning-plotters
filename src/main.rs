use plotters::{prelude::*, style::full_palette::{GREY_100, GREY_300, GREY_600}};

struct PlotStyle {
    bg: RGBColor,
    fg: RGBColor,
}

struct PlotConfig {
    offset: PlotStyle,
    hex: PlotStyle,
    char: PlotStyle,
}

fn do_plot() -> Result<(), Box<dyn std::error::Error>> {
    let config = PlotConfig {
        offset: PlotStyle { bg: GREY_300, fg: GREY_600 },
        hex: PlotStyle { bg: WHITE, fg: BLACK },
        char: PlotStyle { bg: GREY_100, fg: BLACK },
    };

    let mut buf: Vec<_> = vec![0; 3];
    let backend = BitMapBackend::with_buffer(&mut buf, (1, 1));
    let style = TextStyle::from(("Courier New", 18).into_font()).color(&BLACK);
    let (char_width, char_height): (u32, u32) = backend.estimate_text_size("C", &style)?;
    let (hex_width, _): (u32, u32) = backend.estimate_text_size("HH", &style)?;
    let (offset_width, _): (u32, u32) = backend.estimate_text_size("00000000", &style)?;
    println!("offset_width: {}, hex_width: {}, char_width: {}", offset_width, hex_width, char_width);
    drop(backend);
    let hex_view_width = char_width * 16 + hex_width * 16 + char_width * 3;
    let char_view_width = char_width * 16;
    let img_width = offset_width + hex_view_width + char_view_width + char_width;
    let img_height = char_height * 32;
    let mut backend = BitMapBackend::new("target/1.png", (img_width, img_height));
    let (width, height) = backend.get_size();
    println!("request img size: {}x{}, result img size: {}x{}", img_width, img_height, width, height);

    backend.draw_rect((0, 0), (width as i32, height as i32), &config.hex.bg, true)?;
    backend.draw_rect((0, 0), (offset_width as i32, height as i32), &config.offset.bg, true)?;
    backend.draw_rect(((offset_width + hex_view_width) as i32, 0), ((offset_width + hex_view_width + char_view_width) as i32, height as i32), &config.char.bg, true)?;

    let offset_style = style.clone().color(&config.offset.fg);
    for i in 0..256 {
        let line = i / 16;
        if (i % 16) == 0 {
            let offset = format!("{:08X}", line * 16);
            backend.draw_text(&offset, &offset_style, (0, (line * char_height) as i32))?;
        }
        let byte_hex = format!("{:02X}", i % 256);
        let index = i % 16;
        let x = if index < 8 {
            offset_width + char_width + (char_width + hex_width) * index
        } else {
            offset_width + char_width * 2 + (char_width + hex_width) * index
        } as i32;
        backend.draw_text(&byte_hex, &style, (x, (line * char_height) as i32))?;
        let byte_char = {
            let byte = i % 256;
            let c = match char::from_u32(byte as u32) {
                Some(c) => if c.is_ascii_graphic() {
                    c
                } else {
                    '.'
                },
                None => '.',
            };
            format!("{}", c)
        };
        backend.draw_text(&byte_char, &style, ((offset_width + hex_view_width + (i % 16) * char_width) as i32, (line * char_height) as i32))?;
    }

    backend.present()?;
    Ok(())
}

fn main() {
    do_plot().unwrap();
}