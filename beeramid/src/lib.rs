/*
https://www.codewars.com/kata/51e04f6b544cf3f6550000c1

Let's pretend your company just hired your friend from college and paid you a referral bonus. Awesome! To celebrate, you're taking your team out to the terrible dive bar next door and using the referral bonus to buy, and build, the largest three-dimensional beer can pyramid you can. And then probably drink those beers, because let's pretend it's Friday too.

A beer can pyramid will square the number of cans in each level - 1 can in the top level, 4 in the second, 9 in the next, 16, 25...

Complete the beeramid function to return the number of complete levels of a beer can pyramid you can make, given the parameters of:

    your referral bonus, and

    the price of a beer can

For example:

beeramid(1500, 2); // should === 12
beeramid(5000, 3); // should === 16

*/
#[derive(Debug, PartialEq)]
pub enum BeeramidError {
    FreeBeer,
}

fn beeramid_iterative(bonus: usize, beer_can_price: usize) -> Result<usize, BeeramidError> {
    let mut can_count: usize = 0;
    let mut level: usize = 0;
    loop {
        let this_can_count = can_count + usize::pow(level + 1, 2);

        if (this_can_count * beer_can_price) > bonus {
            break;
        }

        level += 1;
        can_count = this_can_count;
    }

    Ok(level)
}

pub fn beeramid(bonus: usize, beer_can_price: usize) -> Result<usize, BeeramidError> {
    match beer_can_price == 0 {
        false => beeramid_iterative(bonus, beer_can_price),
        true => Err(BeeramidError::FreeBeer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_budget() {
        assert_eq!(beeramid(0, 1).unwrap(), 0);
    }

    #[test]
    fn free_beer() {
        assert_eq!(beeramid(usize::MAX, 0).unwrap_err(), BeeramidError::FreeBeer);
    }

    #[test]
    fn craft_beer() {
        assert_eq!(beeramid(140,10).unwrap(), 3);
    }

    #[test]
    fn examples() {
        assert_eq!(beeramid(1500, 2).unwrap(), 12);
        assert_eq!(beeramid(5000, 3).unwrap(), 16);
    }

    #[test]
    fn examples_and_change() {
        assert_eq!(beeramid(1501, 2).unwrap(), 12);
        assert_eq!(beeramid(5001, 3).unwrap(), 16);
    }
}
