use minifb::{Key, Window, WindowOptions};

fn main() {
    let (width, height) = (1000, 600);

    // 画像をウィンドウに表示
    let mut window = Window::new(
        "Image with Plots",
        width as usize,
        height as usize,
        WindowOptions::default(),
        )
        .expect("ウィンドウの作成に失敗しました");

    // 表示する画像をバッファとして作成する
    // 色並びは右の通り ARGB
    let argb: u32 = 255 << 24 | 125 << 16 | 125 << 8 | 125;
    let mut buffer: Vec<u32> = vec![argb; width as usize * height as usize];
    // 一部を別の色で塗りつぶす
    for x in 0..width/2 {
        for y in 0..height/2 {
            let argb_area: u32 = 255 << 24 | 0 << 16 | 0 << 8 | 125;
            buffer[y * width + x] = argb_area;
        }
    }

    // ウィンドウの表示
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("ウィンドウの更新に失敗しました");
    }

    println!("Bye!");
}
