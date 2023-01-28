use crate::rbp::*;

/* Data structure */
pub struct Game {
    pub dice: [i32; 7],
    pub scores: [i32; 13],
}

/* Methods */
impl Game {
    /* Main loop */
    pub fn main_loop(&mut self) {
        self.default_scores();
        let mut count = 0;

        while count < 13 {
            self.default_dice();
            self.dice_count();
            self.player_rerolls();

            if self.rolls_left() {
                self.dice_count();
                self.player_rerolls();
            };

            self.dice_count();
            self.scoring_selection();
            count += 1;
        }

        self.output_scores();
    }

    /* Sets the dice held at the beginning of each round.
    The zero index represents the number of re-rolls. */
    fn default_dice(&mut self) {
        self.dice[0] = 5;
        let mut count = 1;
        while count < 7 {
            self.dice[count] = 0;
            count += 1;
        }
    }

    /* Sets the value of each score box at the beginning of each game.
    A value of '-1' means that the score box has not been filled. */
    fn default_scores(&mut self) {
        let mut count = 0;
        while count < 13 {
            self.scores[count] = -1;
            count += 1;
        }
    }

    /* Rolls the dice and stores the results */
    fn dice_count(&mut self) {
        while self.dice[0] > 0 {
            let roll = pseudo(1, 6);
            self.dice[0] -= 1;

            match roll {
                1 => self.dice[1] += 1,
                2 => self.dice[2] += 1,
                3 => self.dice[3] += 1,
                4 => self.dice[4] += 1,
                5 => self.dice[5] += 1,
                6 => self.dice[6] += 1,
                _ => vt_put_slice("Error"),
            }
        }
    }

    /* Player re-rolls dice by type */
    fn player_rerolls(&mut self) {
        let mut count = 1;
        while count < 7 {
            if self.dice[count] > 0 {
                self.score_card();
                let mut s = String::from("\n\n     How many ");
                s.push_str(&count.to_string());
                s.push_str("s do you wish to re-roll?");
                vt_put_slice(&s);
                let ch = vt_key_i32();
                let num = ch - 48;
                if num > 0 && num <= self.dice[count] {
                    self.dice[count] -= num;
                    self.dice[0] += num;
                }
            }
            count += 1;
        }
    }

    /* Uses the number of re-rolls in the main function. */
    fn rolls_left(&mut self) -> bool {
        self.dice[0] > 0
    }

    /* Scores written to both the file "output.txt", and the screen */
    fn output_scores(&mut self) {
        vt_cls();
        let mut output = String::from("\n\n     FINAL SCORE CARD\n");

        output.push_str("\n     Ones                     ");
        output.push_str(&self.scores[0].to_string());

        output.push_str("\n     Twos                     ");
        output.push_str(&self.scores[1].to_string());

        output.push_str("\n     Threes                   ");
        output.push_str(&self.scores[2].to_string());

        output.push_str("\n     Fours                    ");
        output.push_str(&self.scores[3].to_string());

        output.push_str("\n     Fives                    ");
        output.push_str(&self.scores[4].to_string());

        output.push_str("\n     Sixes                    ");
        output.push_str(&self.scores[5].to_string());

        output.push('\n');
        output.push_str("\n     Upper total              ");
        output.push_str(&self.upper_total().to_string());

        output.push_str("\n     Upper bonus              ");
        output.push_str(&self.upper_bonus().to_string());
        output.push('\n');

        output.push_str("\n     Three of a kind          ");
        output.push_str(&self.scores[6].to_string());

        output.push_str("\n     Four of a kind           ");
        output.push_str(&self.scores[7].to_string());

        output.push_str("\n     Full house               ");
        output.push_str(&self.scores[8].to_string());

        output.push_str("\n     Small Straight           ");
        output.push_str(&self.scores[9].to_string());

        output.push_str("\n     Large Straight           ");
        output.push_str(&self.scores[10].to_string());

        output.push_str("\n     Chance                   ");
        output.push_str(&self.scores[11].to_string());

        output.push_str("\n     Yahtzee                  ");
        output.push_str(&self.scores[12].to_string());

        output.push('\n');
        output.push_str("\n     Grand total              ");
        output.push_str(&self.grand_total().to_string());
        output.push('\n');

        vt_put_slice(&output);
        append_to_file("scores.txt", &output);
    }

    /* Loop to insure that a valid input has been entered */
    fn scoring_selection(&mut self) {
        loop {
            self.score_card();
            vt_put_slice("\n\n     Which score box? ");
            let ch = vt_key_char();

            if ch == 'a' && self.scores[0] == -1 {
                self.scores[0] = self.dice[1];
                break;
            }

            if ch == 'b' && self.scores[1] == -1 {
                self.scores[1] = self.dice[2] * 2;
                break;
            }

            if ch == 'c' && self.scores[2] == -1 {
                self.scores[2] = self.dice[3] * 3;
                break;
            }

            if ch == 'd' && self.scores[3] == -1 {
                self.scores[3] = self.dice[4] * 4;
                break;
            }

            if ch == 'e' && self.scores[4] == -1 {
                self.scores[4] = self.dice[5] * 5;
                break;
            }

            if ch == 'f' && self.scores[5] == -1 {
                self.scores[5] = self.dice[6] * 6;
                break;
            }

            if ch == 'g' && self.scores[6] == -1 {
                self.scores[6] = self.three_of_a_kind();
                break;
            }

            if ch == 'h' && self.scores[7] == -1 {
                self.scores[7] = self.four_of_a_kind();
                break;
            }

            if ch == 'i' && self.scores[8] == -1 {
                self.scores[8] = self.full_house();
                break;
            }

            if ch == 'j' && self.scores[9] == -1 {
                self.scores[9] = self.small_straight();
                break;
            }

            if ch == 'k' && self.scores[10] == -1 {
                self.scores[10] = self.large_straight();
                break;
            }

            if ch == 'l' && self.scores[11] == -1 {
                self.scores[11] = self.chance();
                break;
            }

            if ch == 'm' && self.scores[12] == -1 {
                self.scores[12] = self.yahtzee();
                break;
            }
        }
    }

    /* Prints out the score card so far. */
    fn score_card(&mut self) {
        vt_cls();
        vt_cursor_off();
        let mut s = String::from("\n     SCORE CARD\n");

        s.push_str("\n     Ones                     ");
        if self.scores[0] == -1 {
            s.push('a');
        } else {
            s.push_str(&self.scores[0].to_string());
        }

        s.push_str("\n     Twos                     ");
        if self.scores[1] == -1 {
            s.push('b');
        } else {
            s.push_str(&self.scores[1].to_string());
        }

        s.push_str("\n     Threes                   ");
        if self.scores[2] == -1 {
            s.push('c');
        } else {
            s.push_str(&self.scores[2].to_string());
        }

        s.push_str("\n     Fours                    ");
        if self.scores[3] == -1 {
            s.push('d');
        } else {
            s.push_str(&self.scores[3].to_string());
        }

        s.push_str("\n     Fives                    ");
        if self.scores[4] == -1 {
            s.push('e');
        } else {
            s.push_str(&self.scores[4].to_string());
        }

        s.push_str("\n     Sixes                    ");
        if self.scores[5] == -1 {
            s.push('f');
        } else {
            s.push_str(&self.scores[5].to_string());
        }

        s.push_str("\n     Three of a kind          ");
        if self.scores[6] == -1 {
            s.push('g');
        } else {
            s.push_str(&self.scores[6].to_string());
        }

        s.push_str("\n     Four of a kind           ");
        if self.scores[7] == -1 {
            s.push('h');
        } else {
            s.push_str(&self.scores[7].to_string());
        }

        s.push_str("\n     Full house               ");
        if self.scores[8] == -1 {
            s.push('i');
        } else {
            s.push_str(&self.scores[8].to_string());
        }

        s.push_str("\n     Small straight           ");
        if self.scores[9] == -1 {
            s.push('j');
        } else {
            s.push_str(&self.scores[9].to_string());
        }

        s.push_str("\n     Large straight           ");
        if self.scores[10] == -1 {
            s.push('k');
        } else {
            s.push_str(&self.scores[10].to_string());
        }

        s.push_str("\n     Chance                   ");
        if self.scores[11] == -1 {
            s.push('l');
        } else {
            s.push_str(&self.scores[11].to_string());
        }

        s.push_str("\n     Yahtzee                  ");
        if self.scores[12] == -1 {
            s.push('m');
        } else {
            s.push_str(&self.scores[12].to_string());
        }

        s.push_str("\n\n     DICE HELD\n\n     ");

        s.push_str(&self.dice[1].to_string());
        s.push_str(" 1s, ");

        s.push_str(&self.dice[2].to_string());
        s.push_str(" 2s, ");

        s.push_str(&self.dice[3].to_string());
        s.push_str(" 3s, ");

        s.push_str(&self.dice[4].to_string());
        s.push_str(" 4s, ");

        s.push_str(&self.dice[5].to_string());
        s.push_str(" 5s, ");

        s.push_str("and ");
        s.push_str(&self.dice[6].to_string());
        s.push_str(" 6s. ");

        s.push_str("\n\n     You have ");
        s.push_str(&self.dice[0].to_string());
        s.push_str(" re-rolls.");

        vt_put_slice(&s);
    }

    /* Three of a kind score function */
    fn three_of_a_kind(&mut self) -> i32 {
        let mut value = 0;
        if self.dice[1] > 2
            || self.dice[2] > 2
            || self.dice[3] > 2
            || self.dice[4] > 2
            || self.dice[5] > 2
            || self.dice[6] > 2
        {
            value = self.dice[1]
                + (self.dice[2] * 2)
                + (self.dice[3] * 3)
                + (self.dice[4] * 4)
                + (self.dice[5] * 5)
                + (self.dice[6] * 6);
        }
        value
    }

    /* Four of a kind score function */
    fn four_of_a_kind(&mut self) -> i32 {
        let mut value = 0;
        if self.dice[1] > 3
            || self.dice[2] > 3
            || self.dice[3] > 3
            || self.dice[4] > 3
            || self.dice[5] > 3
            || self.dice[6] > 3
        {
            value = self.dice[1]
                + (self.dice[2] * 2)
                + (self.dice[3] * 3)
                + (self.dice[4] * 4)
                + (self.dice[5] * 5)
                + (self.dice[6] * 6);
        }
        value
    }

    /* Full house score function */
    fn full_house(&mut self) -> i32 {
        let mut value = 0;
        let mut pair_test = false;
        let mut triple_test = false;

        if self.dice[1] == 3
            || self.dice[2] == 3
            || self.dice[3] == 3
            || self.dice[4] == 3
            || self.dice[5] == 3
            || self.dice[6] == 3
        {
            triple_test = true;
        }

        if self.dice[1] == 2
            || self.dice[2] == 2
            || self.dice[3] == 2
            || self.dice[4] == 2
            || self.dice[5] == 2
            || self.dice[6] == 2
        {
            pair_test = true;
        }

        if triple_test && pair_test {
            value = 25;
        }

        if self.dice[1] == 5
            || self.dice[2] == 5
            || self.dice[3] == 5
            || self.dice[4] == 5
            || self.dice[5] == 5
            || self.dice[6] == 5
        {
            value = 25;
        }

        value
    }

    /* Small straight score function */
    fn small_straight(&mut self) -> i32 {
        let mut value = 0;

        if self.dice[1] > 0 && self.dice[2] > 0 && self.dice[3] > 0 && self.dice[4] > 0 {
            value = 30;
        };

        if self.dice[2] > 0 && self.dice[3] > 0 && self.dice[4] > 0 && self.dice[5] > 0 {
            value = 30;
        };

        if self.dice[3] > 0 && self.dice[4] > 0 && self.dice[5] > 0 && self.dice[6] > 0 {
            value = 30;
        };

        value
    }

    /* Large straight score function */
    fn large_straight(&mut self) -> i32 {
        let mut value = 0;

        if self.dice[1] > 0
            && self.dice[2] > 0
            && self.dice[3] > 0
            && self.dice[4] > 0
            && self.dice[5] > 0
        {
            value = 40;
        };

        if self.dice[2] > 0
            && self.dice[3] > 0
            && self.dice[4] > 0
            && self.dice[5] > 0
            && self.dice[6] > 0
        {
            value = 40;
        };

        value
    }

    /* The total of all dice. Regardless of arrangement */
    fn chance(&mut self) -> i32 {
        return self.dice[1]
            + (self.dice[2] * 2)
            + (self.dice[3] * 3)
            + (self.dice[4] * 4)
            + (self.dice[5] * 5)
            + (self.dice[6] * 6);
    }

    /* Yahtzee score function */
    fn yahtzee(&mut self) -> i32 {
        let mut value = 0;

        if self.dice[1] == 5
            || self.dice[2] == 5
            || self.dice[3] == 5
            || self.dice[4] == 5
            || self.dice[5] == 5
            || self.dice[6] == 5
        {
            value = 50;
        };

        value
    }

    /* Total of the upper section */
    fn upper_total(&mut self) -> i32 {
        return self.scores[0]
            + self.scores[1]
            + self.scores[2]
            + self.scores[3]
            + self.scores[4]
            + self.scores[5];
    }

    /* If the upper section total is greater than 62, then a bonus of 35 is awarded */
    fn upper_bonus(&mut self) -> i32 {
        let mut value = 0;

        if self.upper_total() > 62 {
            value = 35;
        }

        value
    }

    /* Total of all scores */
    fn grand_total(&mut self) -> i32 {
        self.upper_total()
            + self.upper_bonus()
            + self.scores[6]
            + self.scores[7]
            + self.scores[8]
            + self.scores[9]
            + self.scores[10]
            + self.scores[11]
            + self.scores[12]
    }
}
