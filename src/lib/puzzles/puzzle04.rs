use std::collections::HashMap;

use super::Puzzle;
use regex::Regex;

struct Puzzle04 {
    input: String,
}

impl Puzzle<(i32, i32)> for Puzzle04 {
    fn build(input: String) -> Self {
        Self { input }
    }

    fn solve(&self) -> (i32, i32) {
        let normalized = self.input.replace(' ', "\n");

        let valid_passports: usize = normalized
            .split("\n\n")
            .map(|raw| Passport::read(raw))
            .filter(|passport| passport.validate_presence())
            .count();

        let valid_passports_strict: usize = normalized
            .split("\n\n")
            .map(|raw| Passport::read(raw))
            .filter(|passport| passport.validate_complete().iter().all(|(_, v)| *v))
            .count();

        (valid_passports as i32, valid_passports_strict as i32)
    }
}

#[derive(Default, Debug)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn read(input: &str) -> Self {
        input
            .split("\n")
            .map(|e| {
                let kv: Vec<&str> = e.split(":").collect();
                (kv[0], kv[1])
            })
            .fold(Self::default(), |p, (k, v)| p.update(k, v))
    }

    fn update(mut self, key: &str, value: &str) -> Self {
        match key {
            "ecl" => self.ecl = Some(value.to_string()),
            "pid" => self.pid = Some(value.to_string()),
            "eyr" => self.eyr = Some(value.parse::<usize>().unwrap()),
            "hcl" => self.hcl = Some(value.to_string()),
            "byr" => self.byr = Some(value.parse::<usize>().unwrap()),
            "iyr" => self.iyr = Some(value.parse::<usize>().unwrap()),
            "hgt" => self.hgt = Some(value.to_string()),
            _ => (),
        }

        self
    }

    fn validate_presence(&self) -> bool {
        let validations: Vec<bool> = vec![
            self.ecl.is_some(),
            self.pid.is_some(),
            self.eyr.is_some(),
            self.hcl.is_some(),
            self.byr.is_some(),
            self.iyr.is_some(),
            self.hgt.is_some(),
        ];

        validations.into_iter().all(|i| i)
    }

    fn validate_complete(&self) -> HashMap<String, bool> {
        let hcl_re = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
        let opts = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        let mut validations: HashMap<String, bool> = HashMap::new();
        validations.insert("pid".into(), Self::validate_pid(&self.pid));
        validations.insert("hgt".into(), Self::validate_hgt(&self.hgt));
        validations.insert("hcl".into(), Self::validate_regex(&self.hcl, hcl_re));
        validations.insert("byr".into(), Self::validate_in_range(&self.byr, 1920, 2020));
        validations.insert("iyr".into(), Self::validate_in_range(&self.iyr, 2010, 2020));
        validations.insert("eyr".into(), Self::validate_in_range(&self.eyr, 2020, 2030));
        validations.insert("ecl".into(), Self::validate_in_collection(&self.ecl, opts));

        validations
    }

    fn validate_hgt(hgt: &Option<String>) -> bool {
        match hgt {
            Some(hgt) => {
                let len = hgt.len();
                if len < 3 {
                    return false;
                }

                let value = hgt[..(len - 2)].parse::<i32>();
                if value.is_err() {
                    return false;
                }

                let value = value.unwrap();
                match &hgt[len - 2..] {
                    "cm" => value >= 150 && value <= 193,
                    "in" => value >= 59 && value <= 76,
                    _ => false,
                }
            }
            None => false,
        }
    }

    fn validate_pid(pid: &Option<String>) -> bool {
        match pid {
            Some(x) => {
                let is_numeric = x.chars().all(|c| c.is_numeric());
                let is_nine_digits = x.len().eq(&9);

                is_numeric && is_nine_digits
            }
            None => false,
        }
    }

    fn validate_in_range(item: &Option<usize>, min: usize, max: usize) -> bool {
        match item {
            Some(x) => *x >= min && *x <= max,
            None => false,
        }
    }

    fn validate_regex(item: &Option<String>, re: Regex) -> bool {
        match item {
            Some(x) => re.is_match(x),
            None => false,
        }
    }

    fn validate_in_collection(item: &Option<String>, collection: &[&str]) -> bool {
        match item {
            Some(x) => collection.contains(&x.as_str()),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            .to_string();

        let puzzle = Puzzle04::build(input);
        let solution = puzzle.solve();

        assert_eq!(solution, (2, 2))
    }

    #[test]
    fn real_input() {
        let input = fs::read_to_string("inputs/puzzle04.input").unwrap();
        let puzzle = Puzzle04::build(input);
        let solution = puzzle.solve();

        assert_eq!(solution, (264, 224))
    }
}
