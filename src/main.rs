use qrcode::QrCode;
use qrcode::render::unicode;
use clap::Parser;

/// Print QR code to the terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Text to encode in the QR code
    #[arg(short, long)]
    text: String,

    /// Use Sixel graphics to render the QR code
    #[arg(short='S', long, action)]
    sixel: bool,

    /// Scale factor for the QR code. (Only used with Sixel graphics)
    #[arg(short, long, default_value_t = 1)]
    scale: usize,
}

fn main(){
    let args = Args::parse();

    let message = &args.text;

    let scale = args.scale;

    let is_sixel = args.sixel;
    if is_sixel {
        render_qr_sixel(message, scale);
    } else {
        render_qr_unicode(message);
    }

}

fn render_qr_unicode(message: &str) {
    let code = QrCode::new(message).unwrap();
    let string = code.render()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", string);
}


fn render_qr_sixel(message: &str, scale: usize) {

    // Encode some data into bits.
    let code = QrCode::new(message).unwrap();

    println!("\n");

    let string2 = code.render().light_color('0').dark_color('1').build();

    // string2 を改行コードで分割
    let lines: Vec<&str> = string2.split("\n").collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let scaled_row = rows * scale;
    let scaled_col = cols * scale;

    // let mut qr_obj  = vec![vec![char; scaled_col]; scaled_row];
    // char型でサイズはscaled_row × scaled_colの二次元配列
    let mut qr_obj = vec![vec!['0'; scaled_col]; scaled_row];

    for i in 0 ..rows {
        for j in 0 ..cols {
            let c = lines[i].chars().nth(j).unwrap();
            for k in 0 ..scale {
                for l in 0 ..scale {
                    qr_obj[i*scale+k][j*scale+l] = c;
                }
            }
        }
    }

    // Sixel エスケープシーケンス開始
    print!("\x1bPq");

    // カラーパレット1番に白をセット
    print!("#1;2;100;100;100");

    // カラーパレット2番に黒をセット
    print!("#2;2;0;0;0");

    // 白をセット
    print!("#1");

    // 0からrows-1までループ
    for i in 0 ..((scaled_row + 1)/6) {
        // 白をセット
        print!("#1");
        for j in 0 ..scaled_col {
            let mut value: u8 = 0;

            let mut row = i * 6;
            for k in 0 ..6 {
                if qr_obj[row][j] == '0' {
                    value |= 1 << k;
                }
                row += 1;

                if row >= scaled_row {
                    break;
                }
            }

            let c = (value + 63) as char;
            print!("{}", c);
        }

        // 黒を描画するために行の先頭に戻す
        print!("$");

        // 黒をセット
        print!("#2");
        for j in 0 ..scaled_col {
            let mut value: u8 = 0;

            let mut row = i * 6;
            for k in 0 ..6 {
                if qr_obj[row][j] == '1' {
                    value |= 1 << k;
                }
                row += 1;

                if row >= scaled_row {
                    break;
                }
            }

            let c = (value + 63) as char;
            print!("{}", c);
        }

        print!("-");
    }

    // Sixel エスケープシーケンス終了
    print!("\x1b\\");

    println!("\n");
}