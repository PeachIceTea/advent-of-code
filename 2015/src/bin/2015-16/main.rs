use serde_derive::Deserialize;
use serde_scan::scan;

#[derive(Deserialize)]
struct MFCSAM {
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

fn main() {
    let input = std::fs::read_to_string("input/16.txt").expect("input should exist");

    let mut aunts = Vec::new();
    for line in input.lines() {
        let mut info = MFCSAM {
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        };
        let (_, props): (i32, Vec<(String, i32)>) =
            scan!("Sue {}: {}: {}, {}: {}, {}: {}" <- line).unwrap();
        for (prop, val) in props {
            match prop.as_str() {
                "children" => info.children = Some(val),
                "cats" => info.cats = Some(val),
                "samoyeds" => info.samoyeds = Some(val),
                "pomeranians" => info.pomeranians = Some(val),
                "akitas" => info.akitas = Some(val),
                "vizslas" => info.vizslas = Some(val),
                "goldfish" => info.goldfish = Some(val),
                "trees" => info.trees = Some(val),
                "cars" => info.cars = Some(val),
                "perfumes" => info.perfumes = Some(val),
                _ => panic!("Unexpected info"),
            }
        }
        aunts.push(info);
    }

    let target = MFCSAM {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let mut matched_aunt = -1;
    let mut matched_aunt_orec = -1;
    for (i, aunt) in aunts.iter().enumerate() {
        if test_aunt(aunt, &target, false) {
            matched_aunt = i as i32 + 1;
        }
        if test_aunt(aunt, &target, true) {
            matched_aunt_orec = i as i32 + 1;
        }
        if matched_aunt == -1 || matched_aunt_orec == -1 {
            continue;
        }
        break;
    }

    println!(
        "Aunt Sue #{} sent the gift if we take the target at face value.",
        matched_aunt
    );
    println!(
        "Aunt Sue #{} sent the gift if we take the outdated retroencabulator into account.",
        matched_aunt_orec
    );
}

fn test_aunt(aunt: &MFCSAM, target: &MFCSAM, outdated_retroencabulator: bool) -> bool {
    if aunt.children.is_some() && aunt.children != target.children {
        return false;
    }
    if aunt.cats.is_some()
        && ((outdated_retroencabulator && aunt.cats <= target.cats)
            || (!outdated_retroencabulator && aunt.cats != target.cats))
    {
        return false;
    }
    if aunt.samoyeds.is_some() && aunt.samoyeds != target.samoyeds {
        return false;
    }
    if aunt.pomeranians.is_some()
        && ((outdated_retroencabulator && aunt.pomeranians >= target.pomeranians)
            || (!outdated_retroencabulator && aunt.pomeranians != target.pomeranians))
    {
        return false;
    }
    if aunt.akitas.is_some() && aunt.akitas != target.akitas {
        return false;
    }
    if aunt.vizslas.is_some() && aunt.vizslas != target.vizslas {
        return false;
    }
    if aunt.goldfish.is_some()
        && ((outdated_retroencabulator && aunt.goldfish >= target.goldfish)
            || (!outdated_retroencabulator && aunt.goldfish != target.goldfish))
    {
        return false;
    }
    if aunt.trees.is_some()
        && ((outdated_retroencabulator && aunt.trees <= target.trees)
            || (!outdated_retroencabulator && aunt.trees != target.trees))
    {
        return false;
    }
    if aunt.cars.is_some() && aunt.cars != target.cars {
        return false;
    }
    if aunt.perfumes.is_some() && aunt.perfumes != target.perfumes {
        return false;
    }

    true
}
