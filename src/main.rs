extern crate image;

use std::fs::File;
use std::mem;
use std::time::Instant;

use image::GenericImage;
use image::DynamicImage;
mod img_data;
use img_data::Pixel;

static BLACK: u8 = 0x00;
static WHITE: u8 = 0xFF;
static simpleThreshold: u8 = 127;

fn construct_internal_data() -> (Vec<Pixel>, u32, u32) {
    let img = image::open("IMG_0788.JPG").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    let (w, h) = img.dimensions();

    let raw_pixel_vec = img.raw_pixels();

    let mut data: Vec<Pixel> = Vec::with_capacity(
        mem::size_of::<Pixel>()
        * img.height() as usize
        * img.width() as usize
    );

    drop(img);

    let mut _r;
    let mut _g;
    let mut _b;
    // let mut _a;
    let mut iter = raw_pixel_vec.into_iter();

    while let Some(v) = iter.next() {
        _r = v;
        _g = iter.next().unwrap();
        _b = iter.next().unwrap();
        // _a = iter.next().unwrap();

        let pixel = Pixel::new(_r, _g, _b, 255, Pixel::to_luma(_r, _g, _b));

        // let pixel = Pixel { 
        //     r: _r,
        //     g: _g,
        //     b: _b,
        //     a: _a,
        //     l: Pixel.to_luma()
        // };
        data.push(pixel);
    }
    println!("veclen: {}", data.len());
    (data, w, h)
}

fn main() {
    let (imgdata, width, height) = construct_internal_data();

    println!("{}", mem::size_of::<Pixel>());
    let mut luma: Vec<Pixel> = Vec::with_capacity(mem::size_of::<Pixel>() * imgdata.len());
    // luma.push(Pixel {r:1, g:1, b:1, a:1,});

    // slow because these [indexes] are bounds checking.
    // https://users.rust-lang.org/t/how-to-avoid-bounds-checking/4433/6
    // for i in 0..imgdata.len() {
    //     let _r = &imgdata[i].r;
    //     let _g = &imgdata[i].g;
    //     let _b = &imgdata[i].b;
    //     luma.push(Pixel {
    //         r: ((*_r as f32) * 0.212) as u8,
    //         g: ((*_g as f32) * 0.715) as u8,
    //         b: ((*_b as f32) * 0.072) as u8,
    //         a: 255,
    //         l: Pixel.to_luma()}
    //     );
    // }
    
    let mut row = 0;
    let mut col = 0;

    println!("width: {}, height: {}", width, height);
    let mut outimg = DynamicImage::new_rgba8(width, height);

    let start_time = Instant::now();
    for p in 0..imgdata.len() {
        // println!("{:?}", imgdata[p]);
        luma.push(imgdata[p].b_or_w());
        let pixe = image::Rgba([luma[p].r, luma[p].g, luma[p].b, 255]);
        outimg.put_pixel(col, row, pixe);
        col += 1;
        if col == width {
            col = 0;
            row += 1;
        }
    }

    let elapsed_time = start_time.elapsed();

    println!("{:?}", elapsed_time);

    println!("Elapsed: {} ms",
             (elapsed_time.as_secs() * 1_000) + (elapsed_time.subsec_nanos() / 1_000_000) as u64);

    let ref mut fout = File::create("output.png").unwrap();
    outimg.save(fout, image::PNG).unwrap();
    println!("Program ended successfully");
}
