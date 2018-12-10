use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Claim {
    id: u32,
    l: u32,
    w: u32,
    x: u32,
    x2: u32,
    y: u32,
    y2: u32,

}

type Claims = Vec<Claim>;
// grid[(x, y)] = claim_count
type Grid = HashMap<(u32, u32), Vec<u32>>;

fn main() {
    let claims: Claims = read_claims();
    let counts: Grid = get_coord_counts(&claims);

    // part 1
    println!("Intersects: {}", get_overlap_count(&counts));
    // part 2
    println!("The One: {}", get_the_one(&counts, &claims));
}

fn read_claims() -> Claims {
    let contents = fs::read_to_string("input/input.txt")
        .expect("Couldn't read input file");
    let mut claims: Vec<Claim> = Vec::new();

    for line in contents.lines() {
        let no_spaces: Vec<&str> = line.split(" ").collect();
        let xy: Vec<&str> = no_spaces[2].split(",").collect();
        let x = convert_u32(&xy[0]);
        let y = convert_u32(&xy[1].replace(":", ""));
        let lw: Vec<&str> = no_spaces[3].split("x").collect();
        let l = convert_u32(&lw[1]);
        let w = convert_u32(&lw[0]);

        claims.push(
            Claim {
                id: convert_u32(&no_spaces[0].replace("#", "")),
                l,
                w: convert_u32(&lw[0]),
                x,
                x2: x + w - 1,
                y,
                y2: y + l - 1,
            }
        );
    }

    claims
}

fn convert_u32(value: &str) -> u32 {
    value.parse::<u32>().unwrap()
}

fn get_coord_counts(claims: &Vec<Claim>) -> Grid {
    let mut grid: Grid = Grid::new();

    for claim in claims {
        for w in 0..claim.w {
            for l in 0..claim.l {
                let coords = (
                    claim.x + w,
                    claim.y + l,
                );
                let claims = grid.entry(coords).or_insert(vec![]);

                claims.push(claim.id);
            }
        }
    }

    grid
}

fn get_overlap_count(grid: &Grid) -> u32 {
    let mut overlaps: u32 = 0;

    for (_, claims) in grid.iter() {
        if claims.len() > 1 {
            overlaps += 1;
        }
    }

    overlaps
}

fn get_the_one(counts: &Grid, claims: &Claims) -> u32 {
    let mut singles: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    let mut the_one: u32 = 0;

    // find the coordinates with only 1 ID on it
    for (coords, count_claims) in counts {
        if count_claims.len() == 1 {
            let coords_list = singles.entry(count_claims[0]).or_insert(vec![]);

            coords_list.push(*coords);
        }
    }

    // find the claim ID that has all of its coordinates
    for claim in claims {
        if let Some(single_coords) = singles.get(&claim.id) {
            let total_coords = claim.w * claim.l;
            let singles_len = single_coords.len() as u32;

            if total_coords == singles_len {
                the_one = claim.id;
                break;
            }
        }
    }

    the_one
}
