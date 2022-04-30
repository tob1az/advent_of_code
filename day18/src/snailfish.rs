use std::fmt;

// snailfish number
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Number(Item, Item);

#[derive(PartialEq, Debug, Clone)]
pub enum Item {
    Digit(u32),
    Nested(Box<Number>),
    Unset,
}

impl Default for Item {
    fn default() -> Self {
        Item::Unset
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Unset => write!(f, "?"),
            Item::Digit(d) => write!(f, "{}", d),
            Item::Nested(n) => write!(f, "{}", n),
        }
    }
}

impl Number {
    pub fn parse(string: &str) -> Option<Box<Self>> {
        let mut nested_numbers = Vec::new();
        for c in string.chars() {
            match c {
                '[' => nested_numbers.push(Box::new(Number::default())),
                '0'..='9' => {
                    let n = c.to_string().parse::<u32>().ok()?;
                    let last = nested_numbers.last_mut()?;
                    if last.0 == Item::Unset {
                        last.0 = Item::Digit(n);
                    } else if last.1 == Item::Unset {
                        last.1 = Item::Digit(n);
                    } else {
                        return None;
                    }
                }
                ',' => {
                    let last = nested_numbers.last()?;
                    if last.0 == Item::Unset {
                        return None;
                    }
                }
                ']' => {
                    let number = nested_numbers.pop()?;
                    if number.1 == Item::Unset {
                        return None;
                    }
                    if let Some(last) = nested_numbers.last_mut() {
                        if last.0 == Item::Unset {
                            last.0 = Item::Nested(number);
                        } else if last.1 == Item::Unset {
                            last.1 = Item::Nested(number);
                        } else {
                            return None;
                        }
                    } else {
                        return Some(number);
                    }
                }
                _ => {
                    return None;
                }
            }
        }
        None
    }

    pub fn magnitude(&self) -> usize {
        self.number_magnitude(self)
    }

    fn number_magnitude(&self, number: &Self) -> usize {
        3 * self.item_magnitude(&number.0) + 2 * self.item_magnitude(&number.1)
    }

    fn item_magnitude(&self, item: &Item) -> usize {
        match item {
            Item::Digit(d) => *d as usize,
            Item::Nested(n) => self.number_magnitude(&n),
            _ => 0,
        }
    }
}

pub fn add(a: Box<Number>, b: Box<Number>) -> Box<Number> {
    let mut sum = Box::new(Number(Item::Nested(a), Item::Nested(b)));
    loop {
        let exploded = explode_number(&mut sum);
        if exploded {
            continue;
        }
        let split = split_number(&mut sum);
        if !exploded && !split {
            break;
        }
    }
    sum
}

struct NumberTraversal<ProcessItem>
where
    ProcessItem: FnMut(&mut Item, u32) -> bool,
{
    process: ProcessItem,
}

impl<ProcessItem> NumberTraversal<ProcessItem>
where
    ProcessItem: FnMut(&mut Item, u32) -> bool,
{
    fn new(process: ProcessItem) -> Self {
        NumberTraversal { process }
    }

    pub fn traverse_number(&mut self, number: &mut Box<Number>) -> bool {
        self.traverse_nested_number(number, 1)
    }

    fn traverse_nested_number(&mut self, number: &mut Box<Number>, level: u32) -> bool {
        self.traverse_item(&mut number.0, level) || self.traverse_item(&mut number.1, level)
    }

    fn traverse_item(&mut self, item: &mut Item, level: u32) -> bool {
        if (self.process)(item, level) {
            return true;
        }
        if let Item::Nested(n) = item {
            self.traverse_nested_number(n, level + 1)
        } else {
            false
        }
    }
}

pub fn split_number(number: &mut Box<Number>) -> bool {
    NumberTraversal::new(|i, _| {
        if let Item::Digit(d) = i {
            if *d > 9 {
                *i = Item::Nested(split_digit(d.clone()));
                return true;
            }
        }
        false
    })
    .traverse_number(number)
}

fn split_digit(d: u32) -> Box<Number> {
    Box::new(Number(Item::Digit(d / 2), Item::Digit(d - d / 2)))
}

pub fn explode_number(number: &mut Box<Number>) -> bool {
    let mut explosive_pair: Option<(u32, u32)> = None;
    // 1-based
    let mut digit_index: usize = 0;
    let mut left_recipient_index: usize = 0;
    let mut right_recipient_index: usize = 0;
    // start explosion
    if NumberTraversal::new(|i, level| {
        match i {
            Item::Digit(_) => digit_index += 1,
            Item::Nested(n) => {
                if level < 4 {
                    return false;
                }
                if let Number(Item::Digit(a), Item::Digit(b)) = **n {
                    explosive_pair = Some((a, b));
                    *i = Item::Digit(0);
                    left_recipient_index = digit_index;
                    // pair was transformed digit => skip it to find the right recipient
                    right_recipient_index = digit_index + 2;
                    return true;
                }
            }
            _ => (),
        }
        false
    })
    .traverse_number(number)
    {
        // update recipient digits of the exploded pair to the left and to the right
        let mut digit_index = 0;
        return NumberTraversal::new(|i, _| {
            if let Item::Digit(d) = i {
                digit_index += 1;
                if digit_index == left_recipient_index {
                    *d += explosive_pair.unwrap().0;
                } else if digit_index == right_recipient_index {
                    *d += explosive_pair.unwrap().1;
                    return true;
                }
            }
            false
        })
        .traverse_number(number);
    }
    false
}
