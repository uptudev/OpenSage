fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let str_rep = args[1].as_str();
        let x = str_rep.parse::<f32>();

        match x {
            Ok(val) => {
                let res = get_rgb_value(val);
                println!("{:02X}{:02X}{:02X}", res.0, res.1, res.2);
            },
            Err(_) => {}
        }

    }

}

const PH_40: (u8, u8, u8) = (0xff, 0x59, 0x29);
const PH_50: (u8, u8, u8) = (0xfa, 0xac, 0x15);
const PH_60: (u8, u8, u8) = (0xe4, 0xe9, 0x3f);
const PH_65: (u8, u8, u8) = (0xb4, 0xb2, 0x31);
const PH_70: (u8, u8, u8) = (0x52, 0xb9, 0x46);
const PH_80: (u8, u8, u8) = (0x00, 0x96, 0x56);
const PH_85: (u8, u8, u8) = (0x00, 0xb1, 0x98);
const PH_NO: (u8, u8, u8) = (0x00, 0x00, 0x00);

fn get_rgb_value(input: f32) -> (u8, u8, u8) {
    // Plants generally like water around 6.5pH,
    // so "acidic" and "basic" are being used loosely here.
    if input == 6.5 {       // If "neutral"
        PH_65
    } else if input < 6.5 { // If acidic
        if input > 6.0 {
            let diffs = get_linear_diffs(6.0, 6.5, input);
            interpolate_rgb_value(diffs, PH_60, PH_65)
        } else if input > 5.0 {
            let diffs = get_linear_diffs(5.0, 6.0, input);
            interpolate_rgb_value(diffs, PH_50, PH_60)
        } else if input > 4.0 {
            let diffs = get_linear_diffs(4.0, 5.0, input);
            interpolate_rgb_value(diffs, PH_40, PH_50)
        } else if input > 0.0 {
            let diffs = get_linear_diffs(0.0, 4.0, input);
            interpolate_rgb_value(diffs, PH_NO, PH_40)
        } else {
            PH_NO
        }
    } else {                // If basic
        if input < 7.0 {
            let diffs = get_linear_diffs(6.5, 7.0, input);
            interpolate_rgb_value(diffs, PH_65, PH_70)
        } else if input < 8.0 {
            let diffs = get_linear_diffs(7.0, 8.0, input);
            interpolate_rgb_value(diffs, PH_70, PH_80)
        } else if input < 8.5 {
            let diffs = get_linear_diffs(8.0, 8.5, input);
            interpolate_rgb_value(diffs, PH_80, PH_85)
        } else if input < 14.0 {
            let diffs = get_linear_diffs(8.5, 14.0, input);
            interpolate_rgb_value(diffs, PH_85, PH_NO)
        } else {
            PH_NO
        }
    }
}

fn get_linear_diffs(x: f32, y: f32, z: f32) -> (f32, f32) {
    // for each 8-bit RGB value, the value at point z is equal to:
    // x * y_diff + y * x_diff
    (f32::abs(z - x) / f32::abs(y - x), f32::abs(y - z) / f32::abs(y - x))
}

fn interpolate_rgb_value(diffs: (f32, f32), x: (u8, u8, u8), y: (u8, u8, u8)) -> (u8, u8, u8) {
    let cast_x = (f32::from(x.0), f32::from(x.1), f32::from(x.2));
    let cast_y = (f32::from(y.0), f32::from(y.1), f32::from(y.2));
    let mut z: (u8, u8, u8) = (0x00, 0x00, 0x00);

    z.0 = (cast_x.0 * diffs.1 + cast_y.0 * diffs.0).round() as u8;
    z.1 = (cast_x.1 * diffs.1 + cast_y.1 * diffs.0).round() as u8;
    z.2 = (cast_x.2 * diffs.1 + cast_y.2 * diffs.0).round() as u8;

    z
}
