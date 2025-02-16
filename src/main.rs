use qrcode::QrCode;
// use qrcode::render::svg;
use qrcode::render::unicode;
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <message>", args[0]);
        return;
    }

    // 1個目の引数を取得(QRコードにする文字列)
    let message = &args[1];

    // 2個目の引数を取得(拡大倍率)
    // TODO 0は排除
    let scale = if args.len() > 2 {
        args[2].parse().unwrap()
    } else {
        1
    };


    // Encode some data into bits.
    let code = QrCode::new(message).unwrap();

    // You can also render it into a string.
    let string = code.render()
        // .light_color(' ')
        // .dark_color('*')
        // .dark_color(svg::Color("#800000"))
        // .light_color(svg::Color("#ffff80"))
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", string);

    println!("\n");

    let string2 = code.render().light_color('0').dark_color('1').build();

    // string2 を改行コードで分割
    let lines: Vec<&str> = string2.split("\n").collect();
    let rows = lines.len();
    let cols = lines[0].len();
    println!("rows:{}", lines.len());
    println!("cols:{}", lines[0].len());

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

    // 文字列"}G}"を出力
    // print!("}}G}}");
    let mut row_final = 0;

    // 0からrows-1までループ
    for i in 0 ..((scaled_row + 1)/6) {
        // lines[i]を出力
        // print!("{}", lines[i]);

        for j in 0 ..scaled_col {
            let mut value: u8 = 0;

            let mut row = i * 6;
            for k in 0 ..6 {
                // if lines[row].chars().nth(j).unwrap() == '0' {
                //     value |= 1 << k;
                // }
                if qr_obj[row][j] == '0' {
                    value |= 1 << k;
                }
                row += 1;

                if row >= scaled_row {
                    break;
                }
            }

            row_final = row;

            let c = (value + 63) as char;
            print!("{}", c);
    
            // // 文字列"}"を出力
            // // 0からrows-1までループ
            // for j in 0 ..rows {
            //     // lines[j][i]を出力
            //     print!("{}", lines[j].chars().nth(i).unwrap());
            // }
            
        }

        print!("-");
    }

    // 0からcols-1までループ

    

    // Sixel エスケープシーケンス終了
    print!("\x1b\\");

    println!("\n");

    println!("row_final:{}", row_final);

}


// black:   \e[40m
//white:   \e[47m