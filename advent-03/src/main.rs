use std::collections::HashMap;
use std::fs;

struct Claim {
    x: u16,
    y: u16,
    l: u16,
    w: u16,
}

fn main() {
    let claims: HashMap<u16, Claim> = read_claims();

    for (id, claim) in claims.iter() {
        println!(
            "{}: ({}, {}) [{}, {}]",
            id,
            claim.x,
            claim.y,
            claim.l,
            claim.w,
        );
    }
}

fn read_claims() -> HashMap<u16, Claim> {
    let contents = fs::read_to_string("input/input.txt")
        .expect("Couldn't read input file");
    let mut claims: HashMap<u16, Claim> = HashMap::new();

    // #<id> @ <x>,<y>: <l>x<w>
    for line in contents.lines() {
        // split on spaces
        //  [0] => #<id>
        //  [1] => @
        //  [2] => <x>,<y>:
        //  [3] => <l>x<w>
        let no_spaces: Vec<&str> = line.split(" ").collect();
        let id: String = no_spaces[0].replace("#", "");
        let xy: Vec<&str> = no_spaces[2].split(",").collect();
        let lw: Vec<&str> = no_spaces[3].split("x").collect();

        claims.insert(
            convert_u16(&id),
            Claim {
                x: convert_u16(&xy[0]),
                y: convert_u16(&xy[1].replace(":", "")),
                l: convert_u16(&lw[0]),
                w: convert_u16(&lw[1]),
            }
        );
    }

    claims
}

fn convert_u16(value: &str) -> u16 {
    value.parse::<u16>().unwrap()
}
