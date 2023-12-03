use regex::Regex;
use std::str::Split;

pub struct CubeSample {
    pub num_greens: i32,
    pub num_reds: i32,
    pub num_blues: i32,
}

impl CubeSample {
    //only 12 red cubes, 13 green cubes, and 14 blue cubes is legal
    pub fn is_valid_sample(&self) -> bool {
        return self.num_reds <= 12 && self.num_greens <= 13 && self.num_blues <= 14;
    }
}

pub struct CubeSamples {
    samples: Vec<CubeSample>,
    pub game_num: i32,
}

impl CubeSamples {
    pub fn has_valid_samples(&self) -> bool {
        for s in &self.samples {
            if s.is_valid_sample() == false {
                return false;
            }
        }
        return true;
    }

    pub fn fewest_number_for_possible_game(&self) -> CubeSample {
        let mut cs = CubeSample {
            num_blues: 0,
            num_reds: 0,
            num_greens: 0,
        };
        for s in &self.samples {
            if s.num_blues > cs.num_blues {
                cs.num_blues = s.num_blues;
            }
            if s.num_reds > cs.num_reds {
                cs.num_reds = s.num_reds;
            }
            if s.num_greens > cs.num_greens {
                cs.num_greens = s.num_greens;
            }
        }

        return cs;
    }
}

// expecting something of the form Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn parse_cube_sample_line(input: &str) -> CubeSamples {
    let pat = Regex::new(r"^Game (\d+):(.*)$").unwrap();

    if let Some(captures) = pat.captures(input) {
        if let Ok(game_num) = captures[1].parse::<i32>() {
            let samples: Vec<&str> = captures[2].split(';').collect();
            if samples.len() > 0 {
                let mut cube_samples = CubeSamples {
                    samples: Vec::new(),
                    game_num,
                };

                for sample in samples {
                    let mut cube_sample = CubeSample {
                        num_blues: 0,
                        num_greens: 0,
                        num_reds: 0,
                    };
                    let cube_sample_strings: Split<char> = sample.split(',');
                    for cst in cube_sample_strings {
                        let cstpat = Regex::new(r"^\s*(\d+)\s+(blue|red|green)\s*$").unwrap();
                        if let Some(cstpat_captures) = cstpat.captures(cst) {
                            let num_cubes = cstpat_captures[1].parse::<i32>().unwrap();
                            if let Some(cube_color) = cstpat_captures.get(2) {
                                let cube_color_str = cube_color.as_str().to_lowercase();
                                if cube_color_str == "red" {
                                    cube_sample.num_reds = num_cubes;
                                } else if cube_color_str == "green" {
                                    cube_sample.num_greens = num_cubes;
                                } else if cube_color_str == "blue" {
                                    cube_sample.num_blues = num_cubes;
                                } else {
                                    panic!(
                                        "Received unknown cube color {} from input {}",
                                        cube_color_str, input
                                    );
                                }
                            } else {
                                panic!("Could not get color from {} for line {}", cst, input);
                            }
                        } else {
                            panic!("Could not get sample from {}", cst);
                        }
                    }
                    cube_samples.samples.push(cube_sample);
                }
                return cube_samples;
            } else {
                panic!(
                    "Input {} did not contain any semi-colon delimmed samples",
                    input
                );
            }
        } else {
            panic!("Could not extract game number from input {}", input);
        }
    } else {
        return panic!("Sample line {} did not start with Game N", input);
    }
}

mod test_cube_sampler {
    use super::*;

    //Zero case
    #[test]
    fn test_can_sample_cube_draw() {
        let result =
            parse_cube_sample_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(3, result.samples.len());
        assert_eq!(1, result.game_num);

        assert_eq!(3, result.samples[0].num_blues);
        assert_eq!(4, result.samples[0].num_reds);
        assert_eq!(0, result.samples[0].num_greens);

        assert_eq!(6, result.samples[1].num_blues);
        assert_eq!(1, result.samples[1].num_reds);
        assert_eq!(2, result.samples[1].num_greens);

        assert_eq!(0, result.samples[2].num_blues);
        assert_eq!(0, result.samples[2].num_reds);
        assert_eq!(2, result.samples[2].num_greens);
    }

    #[test]
    fn test_is_valid_sample() {
        let cs = CubeSample {
            num_reds: 0,
            num_greens: 0,
            num_blues: 0,
        };
        assert_eq!(cs.is_valid_sample(), true);
    }

    #[test]
    fn test_is_not_valid_sample_cos_reds() {
        let cs = CubeSample {
            num_reds: 13,
            num_greens: 0,
            num_blues: 0,
        };
        assert_eq!(cs.is_valid_sample(), false);
    }

    #[test]
    fn test_is_not_valid_sample_cos_greens() {
        let cs = CubeSample {
            num_reds: 12,
            num_greens: 14,
            num_blues: 0,
        };
        assert_eq!(cs.is_valid_sample(), false);
    }

    #[test]
    fn test_is_not_valid_sample_cos_blues() {
        let cs = CubeSample {
            num_reds: 12,
            num_greens: 13,
            num_blues: 15,
        };
        assert_eq!(cs.is_valid_sample(), false);
    }

    #[test]
    fn test_is_valid_game_on_valid_game_input() {
        let result =
            parse_cube_sample_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(result.has_valid_samples(), true);
    }

    #[test]
    fn test_is_valid_game_on_valid_game_invalid_sample_input() {
        let result =
            parse_cube_sample_line("Game 1: 13 blue, 14 red; 1 red, 15 green, 6 blue; 2 green");

        assert_eq!(result.has_valid_samples(), false);
    }

    #[test]
    fn test_fewest_number_for_possible_game_input_1() {
        let result =
            parse_cube_sample_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let fp = result.fewest_number_for_possible_game();
        assert_eq!(fp.num_reds, 4);
        assert_eq!(fp.num_greens, 2);
        assert_eq!(fp.num_blues, 6);
    }

    #[test]
    fn test_fewest_number_for_possible_game_input_2() {
        let result = parse_cube_sample_line(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        );
        let fp = result.fewest_number_for_possible_game();
        assert_eq!(fp.num_reds, 1);
        assert_eq!(fp.num_greens, 3);
        assert_eq!(fp.num_blues, 4);
    }

    #[test]
    fn test_fewest_number_for_possible_game_input_3() {
        let result = parse_cube_sample_line(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        let fp = result.fewest_number_for_possible_game();
        assert_eq!(fp.num_reds, 20);
        assert_eq!(fp.num_greens, 13);
        assert_eq!(fp.num_blues, 6);
    }

    #[test]
    fn test_fewest_number_for_possible_game_input_4() {
        let result = parse_cube_sample_line(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        );
        let fp = result.fewest_number_for_possible_game();
        assert_eq!(fp.num_reds, 14);
        assert_eq!(fp.num_greens, 3);
        assert_eq!(fp.num_blues, 15);
    }

    #[test]
    fn test_fewest_number_for_possible_game_input_5() {
        let result =
            parse_cube_sample_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let fp = result.fewest_number_for_possible_game();
        assert_eq!(fp.num_reds, 6);
        assert_eq!(fp.num_greens, 3);
        assert_eq!(fp.num_blues, 2);
    }
}
