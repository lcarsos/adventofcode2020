use regex::{Regex, Captures};

struct Line {
    min: usize,
    max: usize,
    cha: char,
    password: String,
}

impl Line {
    pub fn new(cap: &Captures) -> Line {
        Line {
            min: cap.name("min").unwrap().as_str().parse::<usize>().unwrap(),
            max: cap.name("max").unwrap().as_str().parse::<usize>().unwrap(),
            cha: cap.name("char").unwrap().as_str().chars().next().unwrap(),
            password: cap.name("pass").unwrap().as_str().to_string(),
        }
    }
}

struct Algo {
    re: Regex
}

impl Algo {
    pub fn new(regex: Regex) -> Algo {
        Algo {
            re: regex
        }
    }

    pub fn algo(&self, raw_line: &str) -> bool {
        let cap = self.re.captures(raw_line).unwrap();
        let line = Line::new(&cap);
        let foo: Vec<&str> = line.password.matches(line.cha).collect();
        let matches =  foo.len();
        matches >= line.min && matches <= line.max
    }
}

fn main() {
    let driver = Algo::new(Regex::new(
        r"(?P<min>\d+)-(?P<max>\d+) (?P<char>\S): (?P<pass>.*$)").unwrap()
    );
    let capture = driver.algo("1-3 a: abcde");

    //println!("min: {}, max: {} char: {} pass: [{}]",
    //    capture.name("min").unwrap().as_str(),
    //    capture.name("max").unwrap().as_str(),
    //    capture.name("char").unwrap().as_str(),
    //    capture.name("pass").unwrap().as_str()
    //    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_example() {
    let driver = Algo::new(
        Regex::new(
            r"(?P<min>\d+)-(?P<max>\d+) (?P<char>\S): (?P<pass>.*$)").unwrap()
    );
    let capture = driver.algo("1-3 a: abcde");
    assert_eq!(true, capture);
    }

    #[test]
    fn test_second_example() {
    let driver = Algo::new(
        Regex::new(
            r"(?P<min>\d+)-(?P<max>\d+) (?P<char>\S): (?P<pass>.*$)").unwrap()
    );
    let capture = driver.algo("1-3 b: cdefg");
    assert_eq!(false, capture);
    }

    #[test]
    fn test_third_example() {
    let driver = Algo::new(
        Regex::new(
            r"(?P<min>\d+)-(?P<max>\d+) (?P<char>\S): (?P<pass>.*$)").unwrap()
    );
    let capture = driver.algo("2-9 c: ccccccccc");
    assert_eq!(true, capture);
    }
}
