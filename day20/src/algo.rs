use eyre::{eyre, Result};

type Pixel = bool;

#[derive(Debug, PartialEq)]
pub struct Image {
    data: Vec<Vec<Pixel>>,
}

#[derive(Debug, PartialEq)]
pub struct Algorithm {
    lookup_table: Vec<Pixel>,
}

fn parse_pixel(pixel_data: char) -> Result<Pixel> {
    match pixel_data {
        '.' => Ok(false),
        '#' => Ok(true),
        _ => Err(eyre!("Bad pixel: {pixel_data}")),
    }
}

impl Image {
    pub fn parse(input: &str) -> Result<Self> {
        let data = input
            .lines()
            .map(|l| l.chars().map(parse_pixel).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<_>>>()?;
        if !data.iter().all(|row| row.len() == data[0].len()) {
            Err(eyre!("Not all rows have the length {}", data[0].len()))
        } else {
            Ok(Self { data })
        }
    }

    pub fn set_pixel(&mut self, row: usize, column: usize, value: Pixel) {
        self.data[column][row] = value;
    }
}

impl Algorithm {
    pub fn parse(input: &str) -> Result<Self> {
        let lookup_table = input.chars().map(parse_pixel).collect::<Result<Vec<_>>>()?;
        Ok(Self { lookup_table })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_image() {
        assert_eq!(
            Image::parse("...\n###\n.#.").unwrap(),
            Image {
                data: vec![
                    vec![false, false, false],
                    vec![true, true, true],
                    vec![false, true, false],
                ],
            },
        )
    }

    #[test]
    fn image_parsing_fails_on_bad_pixels() {
        assert!(Image::parse("...0").is_err());
    }

    #[test]
    fn image_parsing_fails_on_jagged_image_rows() {
        assert!(Image::parse("...\n##").is_err());
    }

    #[test]
    fn can_parse_algorithm() {
        assert_eq!(
            Algorithm::parse("#.#.").unwrap(),
            Algorithm {
                lookup_table: vec![true, false, true, false]
            }
        );
    }

    #[test]
    fn algorithm_parsing_fails_on_bad_pixel() {
        assert!(Algorithm::parse("#.#.x").is_err());
    }
}
