use aoc_driver::*;

fn solution(i: &str) -> String {
    let mut s = 0;
    for l in i.split_whitespace() {
        let mut digits = vec![];

        for c in l.chars() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap())
            }
        }
        s += digits.first().unwrap() * 10;
        s += digits.last().unwrap();
    }

    println!("{s}");
    unimplemented!("TODO:");
    s.to_string();
}

fn solution2(i: &str) -> String {
    let mut s = 0;
    for l in i.split_whitespace() {
        let mut digits = vec![];

        let mut ch = l.chars();
        while let Some(c) = ch.next() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap())
            } else {
                let mut adv = ch.clone();
                if let Some(v) = match c.to_ascii_lowercase() {
                    'o' => {
                        if adv.next().unwrap() == 'n' && adv.next().unwrap() == 'e' {
                            Some(1)
                        } else {
                            None
                        }
                    }
                    't' => {
                        if let Some(val) = adv.next() {
                            match val {
                                'w' => {
                                    if adv.next().unwrap_or_default() == 'o'
                                        && adv.next().unwrap_or_default() == 'e'
                                    {
                                        Some(2)
                                    } else {
                                        None
                                    }
                                }
                                'h' => {
                                    if adv.next().unwrap_or_default() == 'r'
                                        && adv.next().unwrap_or_default() == 'e'
                                        && adv.next().unwrap_or_default() == 'e'
                                    {
                                        Some(3)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    'f' => {
                        if let Some(val) = adv.next() {
                            match
                        }
                        if adv.next().unwrap_or_default() == 'o'
                            && adv.next().unwrap_or_default() == 'u'
                            && adv.next().unwrap_or_default() == 'r'
                        {
                            Some(4)
                        } else {
                            None
                        }
                    }
                    's' => {
                        if adv.next().unwrap_or_default() == 'i'
                            && adv.next().unwrap_or_default() == 'x'
                        {
                            Some(1)
                        } else {
                            None
                        }
                    }
                    'e' => {
                        if adv.next().unwrap_or_default() == 'i'
                            && adv.next().unwrap_or_default() == 'g'
                            && adv.next().unwrap_or_default() == 'h'
                            && adv.next().unwrap_or_default() == 't'
                        {
                            Some(8)
                        } else {
                            None
                        }
                    }
                    'n' => {
                        if adv.next().unwrap_or_default() == 'i'
                            && adv.next().unwrap_or_default() == 'n'
                            && adv.next().unwrap_or_default() == 'e'
                        {
                            Some(9)
                        } else {
                            None
                        }
                    }
                    _ => None,
                } {
                    digits.push(v);
                    ch = adv;
                }
            }
        }

        for c in l.chars() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap())
            }
        }
        s += digits.first().unwrap() * 10;
        s += digits.last().unwrap();
    }

    println!("{s}");
    unimplemented!("TODO:");
    s.to_string();
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2023:1:2, solution2).unwrap();
    println!("Hello, world!");
}
