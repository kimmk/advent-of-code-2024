use std::collections::HashMap;

struct Rule {
    a : i32,
    b : i32
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        return self.a == other.a && self.b == other.b;
    }
}

fn parse_rules(
    rules_str: &str
) -> Vec<Rule> {
    let mut rules = Vec::new();
    let rule_lines = rules_str.lines();
    for rule_line in rule_lines {
        let (rule_a, rule_b) = rule_line.split_once("|").unwrap();
        let rule = Rule {
            a: rule_a.parse().unwrap(),
            b: rule_b.parse().unwrap(),
        };
        rules.push(rule);
    }
    return rules;
}

fn analyze_pages(
    pages: &Vec<i32>,
    rules: &Vec<Rule>,
) -> bool {
    let mut page_order = HashMap::<i32, i32>::new();
    for (idx, page) in pages.iter().enumerate() {
        page_order.insert(*page, idx as i32);
    }
    for rule in rules {
        let a = page_order.get(&rule.a);
        let b = page_order.get(&rule.b);
        if a.is_none() || b.is_none() {
            continue;
        }
        if a.unwrap() > b.unwrap() {
            print!("breaks rule {}|{} ", rule.a, rule.b);
            return false;
        }
    }
    return true;
}

fn sort_pages(
    pages: &mut Vec<i32>,
    rules: &Vec<Rule>,
) {
    pages.sort_by(
        |page_a, page_b| {
            if rules.contains(&Rule { a: *page_b, b: *page_a }) {
                print!("swap {}|{} ", page_a, page_b);
                return std::cmp::Ordering::Less;
            }
            return std::cmp::Ordering::Equal;
        }
    );
    pages.reverse();
}

fn main() {
    let input = include_str!("input");
    let (rules_str, page_list_str) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);
    let mut score = 0;
    for page_list in page_list_str.lines() {
        let mut pages: Vec<i32> = page_list.split(",").map(|page| page.parse::<i32>().unwrap()).collect();
        print!("pages {:?}", pages);
        if analyze_pages(&pages, &rules) {
            println!("valid");
        }
        else {
            sort_pages(&mut pages, &rules);
            score += pages[pages.len() / 2];
            println!("sorted: {:?}", pages);
        }
    }
    println!("score: {}", score);
}
