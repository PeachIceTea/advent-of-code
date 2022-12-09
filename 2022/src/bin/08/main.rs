fn main() {
    let input = std::fs::read_to_string("input/08.txt").unwrap();

    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_string().parse::<i32>().unwrap());
        }
        map.push(row);
    }

    let mut visible_trees = map.len() * 2 + (map[0].len() - 2) * 2;
    let mut highest_scenic_score = 0;
    for tree_x in 1..map.len() - 1 {
        for tree_y in 1..map[tree_x].len() - 1 {
            let tree_size = map[tree_x][tree_y];

            let mut top = 0;
            let mut can_see_top_edge = true;
            for check_x in (0..tree_x).rev() {
                top += 1;
                if map[check_x][tree_y] >= tree_size {
                    can_see_top_edge = false;
                    break;
                }
            }

            let mut bottom = 0;
            let mut can_see_bottom_edge = true;
            for check_x in tree_x + 1..map.len() {
                bottom += 1;
                if map[check_x][tree_y] >= tree_size {
                    can_see_bottom_edge = false;
                    break;
                }
            }

            let mut left = 0;
            let mut can_see_left_edge = true;
            for check_y in (0..tree_y).rev() {
                left += 1;
                if map[tree_x][check_y] >= tree_size {
                    can_see_left_edge = false;
                    break;
                }
            }

            let mut right = 0;
            let mut can_see_right_edge = true;
            for check_y in tree_y + 1..map[tree_x].len() {
                right += 1;
                if map[tree_x][check_y] >= tree_size {
                    can_see_right_edge = false;
                    break;
                }
            }

            if can_see_top_edge || can_see_bottom_edge || can_see_left_edge || can_see_right_edge {
                visible_trees += 1;
            }

            let score = top * bottom * left * right;
            if score > highest_scenic_score {
                highest_scenic_score = score;
            }
        }
    }

    println!(
        "There are {} tree visible from the outside the grid.",
        visible_trees
    );
    println!(
        "The highest scenic score for any tree is {}.",
        highest_scenic_score
    );
}
