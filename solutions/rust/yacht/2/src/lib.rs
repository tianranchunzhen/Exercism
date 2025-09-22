#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(mut dice: Dice, category: Category) -> u8 {
    dice.sort_unstable();
    match category {
        Category::Ones => dice.iter().filter(|&&num| num == 1).count() as u8 * 1,
        Category::Twos => dice.iter().filter(|&&num| num == 2).count() as u8 * 2,
        Category::Threes => dice.iter().filter(|&&num| num == 3).count() as u8 * 3,
        Category::Fours => dice.iter().filter(|&&num| num == 4).count() as u8 * 4,
        Category::Fives => dice.iter().filter(|&&num| num == 5).count() as u8 * 5,
        Category::Sixes => dice.iter().filter(|&&num| num == 6).count() as u8 * 6,
        Category::FullHouse => {
            if (dice[0] == dice[2] && dice[3] == dice[4] && dice[0] != dice[4]) || (dice[0] == dice[1] && dice[2] == dice[4] && dice[0] != dice[4]) {
                dice.iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            if dice[0] == dice[3] {
                4 * dice[0]
            } else if dice[1] == dice[4] {
                4 * dice[1]
            } else {
                0
            }
        }
        Category::LittleStraight => {
            if dice == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if dice == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if dice[0] == dice[4] {
                50
            } else {
                0
            }
        }
    }
}
