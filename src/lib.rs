use entropy_calculator::get_file_entropy;
use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub enum PredictingMode {
    NONE,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    NEW,
}

pub fn test_entropies(path: &str) {
    let (r_none, g_none, b_none) = calculate_entropies(path, PredictingMode::NONE);
    let (r_one, g_one, b_one) = calculate_entropies(path, PredictingMode::ONE);
    let (r_two, g_two, b_two) = calculate_entropies(path, PredictingMode::TWO);
    let (r_three, g_three, b_three) = calculate_entropies(path, PredictingMode::THREE);
    let (r_four, g_four, b_four) = calculate_entropies(path, PredictingMode::FOUR);
    let (r_five, g_five, b_five) = calculate_entropies(path, PredictingMode::FIVE);
    let (r_six, g_six, b_six) = calculate_entropies(path, PredictingMode::SIX);
    let (r_seven, g_seven, b_seven) = calculate_entropies(path, PredictingMode::SEVEN);
    let (r_new, g_new, b_new) = calculate_entropies(path, PredictingMode::NEW);

    let whole_none = (r_none + g_none + b_none) / 3.0;
    let whole_one = (r_one + g_one + b_one) / 3.0;
    let whole_two = (r_two + g_two + b_two) / 3.0;
    let whole_three = (r_three + g_three + b_three) / 3.0;
    let whole_four = (r_four + g_four + b_four) / 3.0;
    let whole_five = (r_five + g_five + b_five) / 3.0;
    let whole_six = (r_six + g_six + b_six) / 3.0;
    let whole_seven = (r_seven + g_seven + b_seven) / 3.0;
    let whole_new = (r_new + g_new + b_new) / 3.0;

    println!("Entropies (whole, red, green, blue).");
    println!("Image: \t{:.2} {:.2} {:.2} {:.2}", whole_none, r_none, g_none, b_none);
    println!("One: \t{:.2} {:.2} {:.2} {:.2}", whole_one, r_one, g_one, b_one);
    println!("Two: \t{:.2} {:.2} {:.2} {:.2}", whole_two, r_two, g_two, b_two);
    println!("Three: \t{:.2} {:.2} {:.2} {:.2}", whole_three, r_three, g_three, b_three);
    println!("Four: \t{:.2} {:.2} {:.2} {:.2}", whole_four, r_four, g_four, b_four);
    println!("Five: \t{:.2} {:.2} {:.2} {:.2}", whole_five, r_five, g_five, b_five);
    println!("Six: \t{:.2} {:.2} {:.2} {:.2}", whole_six, r_six, g_six, b_six);
    println!("Seven: \t{:.2} {:.2} {:.2} {:.2}", whole_seven, r_seven, g_seven, b_seven);
    println!("New: \t{:.2} {:.2} {:.2} {:.2}", whole_new, r_new, g_new, b_new);

    let whole_prediction = [
        (whole_none, PredictingMode::NONE),
        (whole_one, PredictingMode::ONE),
        (whole_two, PredictingMode::TWO),
        (whole_three, PredictingMode::THREE),
        (whole_four, PredictingMode::FOUR),
        (whole_five, PredictingMode::FIVE),
        (whole_six, PredictingMode::SIX),
        (whole_seven, PredictingMode::SEVEN),
        (whole_new, PredictingMode::NEW),
    ]
    .iter()
    .map(|(n, m)| ((n * 1_000_000.0) as u32, *m))
    .min_by(|(n1, _), (n2, _)| n1.cmp(n2))
    .unwrap()
    .1;

    let r_prediction = [
        (r_none, PredictingMode::NONE),
        (r_one, PredictingMode::ONE),
        (r_two, PredictingMode::TWO),
        (r_three, PredictingMode::THREE),
        (r_four, PredictingMode::FOUR),
        (r_five, PredictingMode::FIVE),
        (r_six, PredictingMode::SIX),
        (r_seven, PredictingMode::SEVEN),
        (r_new, PredictingMode::NEW),
    ]
    .iter()
    .map(|(n, m)| ((n * 1_000_000.0) as u32, *m))
    .min_by(|(n1, _), (n2, _)| n1.cmp(n2))
    .unwrap()
    .1;

    let g_prediction = [
        (g_none, PredictingMode::NONE),
        (g_one, PredictingMode::ONE),
        (g_two, PredictingMode::TWO),
        (g_three, PredictingMode::THREE),
        (g_four, PredictingMode::FOUR),
        (g_five, PredictingMode::FIVE),
        (g_six, PredictingMode::SIX),
        (g_seven, PredictingMode::SEVEN),
        (g_new, PredictingMode::NEW),
    ]
    .iter()
    .map(|(n, m)| ((n * 1_000_000.0) as u32, *m))
    .min_by(|(n1, _), (n2, _)| n1.cmp(n2))
    .unwrap()
    .1;

    let b_prediction = [
        (b_none, PredictingMode::NONE),
        (b_one, PredictingMode::ONE),
        (b_two, PredictingMode::TWO),
        (b_three, PredictingMode::THREE),
        (b_four, PredictingMode::FOUR),
        (b_five, PredictingMode::FIVE),
        (b_six, PredictingMode::SIX),
        (b_seven, PredictingMode::SEVEN),
        (b_new, PredictingMode::NEW),
    ]
    .iter()
    .map(|(n, m)| ((n * 1_000_000.0) as u32, *m))
    .min_by(|(n1, _), (n2, _)| n1.cmp(n2))
    .unwrap()
    .1;

    println!();
    println!("Best prediction for whole is {:?}", whole_prediction);
    println!("Best prediction for red is {:?}", r_prediction);
    println!("Best prediction for green is {:?}", g_prediction);
    println!("Best prediction for blue is {:?}", b_prediction);
}

pub fn calculate_entropies<P>(path: P, predicting_mode: PredictingMode) -> (f64, f64, f64)
where
    P: AsRef<std::path::Path>,
{
    let black: Rgb<u8> = Rgb::from([0, 0, 0]);

    let img = image::open(path).expect("couldn't open an image");
    let rgb_img = img.to_rgb8();

    let mut r_diffs = Vec::with_capacity(img.width() as usize * img.height() as usize);
    let mut g_diffs = Vec::with_capacity(img.width() as usize * img.height() as usize);
    let mut b_diffs = Vec::with_capacity(img.width() as usize * img.height() as usize);

    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        let mut n = black;
        let mut w = black;
        let mut nw = black;

        if x > 0 {
            w = *rgb_img.get_pixel(x - 1, y);
        }

        if y > 0 {
            n = *rgb_img.get_pixel(x, y - 1);
        }

        if x > 0 && y > 0 {
            nw = *rgb_img.get_pixel(x - 1, y - 1);
        }

        let [n_r, n_g, n_b] = n.0;
        let [w_r, w_g, w_b] = w.0;
        let [nw_r, nw_g, nw_b] = nw.0;

        let [other_r, other_g, other_b] = match predicting_mode {
            PredictingMode::NONE => black.0,
            PredictingMode::ONE => w.0,
            PredictingMode::TWO => n.0,
            PredictingMode::THREE => nw.0,
            PredictingMode::FOUR => [n_r + w_r - nw_r, n_g + w_g - nw_g, n_b + w_b - nw_b],
            PredictingMode::FIVE => [
                n_r + (w_r - nw_r) / 2,
                n_g + (w_g - nw_g) / 2,
                n_b + (w_b - nw_b) / 2,
            ],
            PredictingMode::SIX => [
                w_r + (n_r - nw_r) / 2,
                w_g + (n_g - nw_g) / 2,
                w_b + (n_b - nw_b) / 2,
            ],
            PredictingMode::SEVEN => [(n_r + w_r) / 2, (n_g + w_g) / 2, (n_b + w_b) / 2],
            PredictingMode::NEW => calculate_max_prediction(n, w, nw),
        };

        let [pixel_r, pixel_g, pixel_b] = pixel.0;

        let r_diff = pixel_r - other_r;
        let g_diff = pixel_g - other_g;
        let b_diff = pixel_b - other_b;

        r_diffs.push(r_diff);
        g_diffs.push(g_diff);
        b_diffs.push(b_diff);
    }

    let rs_entropy = calculate_color_entropy(&r_diffs);
    let gs_entropy = calculate_color_entropy(&g_diffs);
    let bs_entropy = calculate_color_entropy(&b_diffs);

    (rs_entropy, gs_entropy, bs_entropy)
}

fn calculate_max_prediction(n: Rgb<u8>, w: Rgb<u8>, nw: Rgb<u8>) -> [u8; 3] {
    let [n_r, n_g, n_b] = n.0;
    let [w_r, w_g, w_b] = w.0;
    let [nw_r, nw_g, nw_b] = nw.0;

    let r = if nw_r >= w_r.max(n_r) {
        w_r.max(n_r)
    } else if nw_r <= w_r.min(n_r) {
        w_r.min(n_r)
    } else {
        w_r + n_r - nw_r
    };

    let g = if nw_g >= w_g.max(n_g) {
        w_g.max(n_g)
    } else if nw_g <= w_g.min(n_g) {
        w_g.min(n_g)
    } else {
        w_g + n_g - nw_g
    };

    let b = if nw_b >= w_b.max(n_b) {
        w_b.max(n_b)
    } else if nw_b <= w_b.min(n_b) {
        w_b.min(n_b)
    } else {
        w_b + n_b - nw_b
    };

    [r, g, b]
}

fn calculate_color_entropy(color: &[u8]) -> f64 {
    std::fs::write("color_helper", color).unwrap();

    let entropy = get_file_entropy(&std::path::PathBuf::from("color_helper"));

    std::fs::remove_file("color_helper").expect("file doesnt exist");

    entropy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_image_works() {
        calculate_entropies("testy/example0.tga", PredictingMode::ONE);
    }
}
