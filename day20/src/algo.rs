use eyre::{eyre, Result};
use itertools::Itertools;
use std::fmt;
use crate::data;


#[derive(Debug, PartialEq, Clone, Copy)]
enum Pixel {
    Light,
    Dark,
}

#[derive(Debug, PartialEq)]
pub struct Image {
    data: Vec<Vec<Pixel>>,
    outer_pixel_state: Pixel,
    rows: i32,
    columns: i32,
}

#[derive(Debug, PartialEq)]
pub struct Algorithm {
    lookup_table: Vec<Pixel>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in -1..=self.data.len() as i32 {
            for column in -1..=self.data[0].len() as i32 {
                match self.get_pixel(row, 0, column, 0) {
                    Pixel::Dark => write!(f, "."),
                    Pixel::Light => write!(f, "#"),
                };
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

fn parse_pixel(pixel_data: char) -> Result<Pixel> {
    match pixel_data {
        '.' => Ok(Pixel::Dark),
        '#' => Ok(Pixel::Light),
        _ => Err(eyre!("Bad pixel: {pixel_data}")),
    }
}

fn flip_pixel(pixel: Pixel) -> Pixel {
    match pixel {
        Pixel::Dark => Pixel::Light,
        Pixel::Light => Pixel::Dark,
    }
}

impl Image {
    fn new(data: Vec<Vec<Pixel>>) -> Self {
        Self::with_outer_pixel(data, Pixel::Dark)
    }

    fn with_outer_pixel(data: Vec<Vec<Pixel>>, outer_pixel_state: Pixel) -> Self {
        debug_assert!(!data.is_empty() && !data[0].is_empty());
        Self {
            outer_pixel_state,
            rows: data.len() as i32,
            columns: data[0].len() as i32,
            data: data,
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        if input.is_empty() {
            return Err(eyre!("empty image string"));
        }
        let data = input
            .lines()
            .map(|l| l.chars().map(parse_pixel).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<_>>>()?;
        if !data.iter().all(|row| row.len() == data[0].len()) {
            Err(eyre!("Not all rows have the length {}", data[0].len()))
        } else {
            Ok(Self::new(data))
        }
    }

    fn get_pixel_box_factor(&self, row: i32, column: i32) -> usize {
        let mut bit = 1usize;
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|(r, c)| {
            self.get_pixel(row, *r, column, *c)})
        .rev()
        .fold(0usize, |factor, pixel| {
            let result = match pixel {
                Pixel::Light => factor + bit,
                Pixel::Dark => factor,
            };
            bit <<= 1;
            result
        })
    }

    fn get_pixel(
        &self,
        base_row: i32,
        row_offset: i32,
        base_column: i32,
        column_offset: i32,
    ) -> Pixel {
        let row = base_row + row_offset;
        if row < 0 || row >= self.rows {
            return self.outer_pixel_state;
        }
        let column = base_column + column_offset;
        if column < 0 || column >= self.columns {
            return self.outer_pixel_state;
        }
        self.data[row as usize][column as usize]
    }

    pub fn count_light_pixels(&self) -> Result<usize> {
        if self.outer_pixel_state == Pixel::Light {
            Err(eyre!("Infinite number of light pixels!"))
        } else {
            Ok(self
                .data
                .iter()
                .map(|r| r.iter().filter(|p| **p == Pixel::Light).count())
                .sum())
        }
    }
}

impl Algorithm {
    pub fn parse(input: &str) -> Result<Self> {
        const TABLE_SIZE: usize = 512;
        if input.len() != TABLE_SIZE {
            return Err(eyre!(
                "Wrong algorithm data size {}, must be {TABLE_SIZE}",
                input.len()
            ));
        }
        let lookup_table = input.chars().map(parse_pixel).collect::<Result<Vec<_>>>()?;

        Ok(Self { lookup_table })
    }

    fn enhance_pixel(&self, box_factor: usize) -> Pixel {
        debug_assert!(box_factor < self.lookup_table.len());
        self.lookup_table[box_factor]
    }

    pub fn enhance_image(&self, image: &Image) -> Image {
        let new_column_count = image.columns + 2;
        let enhanced_data = (-1..=image.rows)
            .cartesian_product(-1..=image.columns)
            .map(|(r, c)| self.enhance_pixel(image.get_pixel_box_factor(r, c)))
            .chunks(new_column_count as usize)
            .into_iter()
            .map(|r| r.collect_vec())
            .collect_vec();
        let outer_pixel_state = self.enhance_pixel(image.get_pixel_box_factor(-2, -2));
        Image::with_outer_pixel(enhanced_data, outer_pixel_state)
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
                    vec![Pixel::Dark, Pixel::Dark, Pixel::Dark],
                    vec![Pixel::Light, Pixel::Light, Pixel::Light],
                    vec![Pixel::Dark, Pixel::Light, Pixel::Dark],
                ],
                outer_pixel_state: Pixel::Dark,
                rows: 3,
                columns: 3
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
    fn can_get_pixel_box_factor() {
        let image = Image::parse("...\n#..\n.#.").unwrap();
        // get center box factor
        println!("{:09b} vs {:09b}", image.get_pixel_box_factor(1, 1), 34);
        assert_eq!(image.get_pixel_box_factor(1, 1), 34);
    }

    #[test]
    fn can_parse_algorithm() {
        assert_eq!(
            Algorithm::parse("#.#.".repeat(128).as_str()).unwrap(),
            Algorithm {
                lookup_table: [Pixel::Light, Pixel::Dark, Pixel::Light, Pixel::Dark].repeat(128)
            }
        );
    }

    #[test]
    fn algorithm_parsing_fails_on_bad_pixel() {
        assert!(Algorithm::parse("#.#x".repeat(128).as_str()).is_err());
    }

    #[test]
    fn algorithm_parsing_fails_if_string_is_not_512_chars() {
        assert!(Algorithm::parse("#.#.".repeat(129).as_str()).is_err());
    }
}
