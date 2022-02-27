use crate::{color, color::{Color, color_constants}, image::Image};

fn rotate((x, y): (i32, i32), (x_center, y_center): (i32, i32), angle_rads: f32) -> (i32, i32) {
    (
        ((x - x_center) as f32 * angle_rads.cos() - (y - y_center) as f32 * angle_rads.sin()) as i32 + x_center,
        ((x - x_center) as f32 * angle_rads.sin() + (y - y_center) as f32 * angle_rads.cos()) as i32 + y_center      
    ) 
}

fn amongus(img: &mut Image, (x, y): (i32, i32), (width, height): (i32, i32), angle_rads: f32, iters: usize, total_iters: usize) {
    if iters == 0 {return}

    let color_lerp = iters as f32 / total_iters as f32 * 255f32;
    let brightness_lerp = 1f32 - (iters as f32 / total_iters as f32);
    let color = color!((brightness_lerp * (255f32 - color_lerp)) as u8, (brightness_lerp * color_lerp) as u8, 0);
    let lerped_cyan = color!(0, (brightness_lerp * 255f32) as u8, (brightness_lerp * 255f32) as u8);
    // Upper shell of the astronaut
    img.draw_line(
        rotate((x - width / 2, y - height / 2), (x, y), angle_rads),
        rotate((x - width / 2, y + height / 2), (x, y), angle_rads),
        color
    );
    img.draw_line(
        rotate((x - width / 2, y + height / 2), (x, y), angle_rads),
        rotate((x + width / 2, y + height / 2), (x, y), angle_rads),
        color
    );
    img.draw_line(
        rotate((x + width / 2, y - height / 2), (x, y), angle_rads),
        rotate((x + width / 2, y + height / 2), (x, y), angle_rads),
        color
    );

    // Legs
    img.draw_line(
        rotate((x - width / 2, y - height / 2), (x, y), angle_rads),
        rotate((x - width / 6, y - height / 2), (x, y), angle_rads),
        color
    );
    img.draw_line(
        rotate((x - width / 6, y - height / 2), (x, y), angle_rads),
        rotate((x - width / 6, y - height / 4), (x, y), angle_rads),
        color
    );
    img.draw_line(
        rotate((x - width / 6, y - height / 4), (x, y), angle_rads),
        rotate((x + width / 6, y - height / 4), (x, y), angle_rads),
        color
    );
    img.draw_line(
        rotate((x + width / 6, y - height / 2), (x, y), angle_rads),
        rotate((x + width / 6, y - height / 4), (x, y), angle_rads),
        color
    );
    img.draw_line(
        rotate((x + width / 6, y - height / 2), (x, y), angle_rads),
        rotate((x + width / 2, y - height / 2), (x, y), angle_rads),
        color
    );

    // Backpack
    img.draw_line(
       rotate((x + width / 2, y + 3 * height / 10), (x, y), angle_rads),
       rotate((x + 4 * width / 5, y + 3 * height / 10), (x, y), angle_rads),
        color
    );
    img.draw_line(
        rotate((x + width / 2, y - height / 5), (x, y), angle_rads),
        rotate((x + 4 * width / 5, y - height / 5), (x, y), angle_rads),
         color
     ); 
    img.draw_line(
    rotate((x + 4 * width / 5, y - height / 5), (x, y), angle_rads),
    rotate((x + 4 * width / 5, y + 3 * height / 10), (x, y), angle_rads),
    color
    ); 

    // Visor
    img.draw_line(
        rotate((x - 2 * width / 5, y + 2 * height / 5), (x, y), angle_rads),
        rotate((x + 2 * width / 5, y + 2 * height / 5), (x, y), angle_rads),
        lerped_cyan
    );
    img.draw_line(
        rotate((x - 2 * width / 5, y + height / 5), (x, y), angle_rads),
        rotate((x + 2 * width / 5, y + height / 5), (x, y), angle_rads),
        lerped_cyan
    );
    img.draw_line(
        rotate((x - 2 * width / 5, y + height / 5), (x, y), angle_rads),
        rotate((x - 2 * width / 5, y + 2 * height / 5), (x, y), angle_rads),
        lerped_cyan
    );
    img.draw_line(
        rotate((x + 2 * width / 5, y + height / 5), (x, y), angle_rads),
        rotate((x + 2 * width / 5, y + 2 * height / 5), (x, y), angle_rads),
        lerped_cyan
    );

    amongus(img, (x, y), (width - width / (iters + 1) as i32, height - height / (iters + 1) as i32), angle_rads + std::f32::consts::PI / (total_iters as f32), iters - 1, total_iters);
}

#[test]
fn spiral_amongla() {
    let mut image = Image::new(500, 500);
    image.set_y_invert(true);
    amongus(&mut image, (250, 250), (225, 400), std::f32::consts::PI, 25, 25);
    image.write_file_test("spiral_amongla").expect("Spiral amongla file write failed");
}

#[test]
fn dw_test() {
    let xres = 500;
    let yres = 500;
    let mut img = Image::new(xres, yres);

    img.set_y_invert(true);
    
    let xresint = xres as i32;
    let yresint = yres as i32;
    // 1 and 5
    img.draw_line((0, 0), (xresint - 1, yresint - 1), color_constants::GREEN);
    img.draw_line((0, 0), (xresint - 1, yresint / 2), color_constants::GREEN);
    img.draw_line((xresint - 1, yresint - 1), (0, yresint / 2), color_constants::GREEN);

    // 8 and 4
    img.draw_line((0, yresint - 1), (xresint - 1, 0), color_constants::CYAN);
    img.draw_line((0, yresint - 1), (xresint - 1, yresint / 2), color_constants::CYAN);
    img.draw_line((xresint - 1, 0), (0, yresint / 2), color_constants::CYAN);

    // 2 and 6
    img.draw_line((0, 0), (xresint / 2, yresint - 1), color_constants::RED);
    img.draw_line((xresint - 1, yresint - 1), (xresint / 2, 0), color_constants::RED);

    // 7 and 3
    img.draw_line((0, yresint - 1), (xresint / 2, 0), color_constants::PURPLE);
    img.draw_line((xresint - 1, 0), (xresint / 2, yresint - 1), color_constants::PURPLE);

    // horizontal and vertical
    img.draw_line((0, yresint / 2), (xresint - 1, yresint / 2), color_constants::YELLOW);
    img.draw_line((xresint / 2, 0), (xresint / 2, yresint - 1), color_constants::YELLOW);

    img.write_file_test("dw-test-line").expect("Image write to file failed");
}