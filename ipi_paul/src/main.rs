use std::env;
mod strings;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].to_lowercase() == "swaparoundclip" {
        let mut new_args = Vec::new();
        for arg in  env::args().skip(2) {
            new_args.push(arg)
        }
        strings::swap_around_clip(new_args);
    } else if args[1].to_lowercase() == "togglecase" {
        strings::toggle_case(&args[2]);
    } else if args[1].to_lowercase() == "togglecaseclip" {
        strings::toggle_case_clip();
    }
}
