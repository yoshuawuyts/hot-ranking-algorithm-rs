#![feature(plugin)]
#![plugin(clippy)]

#[cfg(test)]
mod test;

extern crate time;

use time::Tm;
use std::cmp;

const HALF_LIFE: i32 = 45000;

pub fn rank (upvotes: &i32, downvotes: &i32, timestamp: Tm, epoch: Tm) -> i32 {
  let seconds = timestamp.tm_nsec - epoch.tm_nsec;
  let score = upvotes - downvotes;
  let upper = cmp::max(score, 1) as f32;
  let order = upper.log(10.0) as i32;

  let sign = match score {
    n if n > 0 => 1,
    n if n < 0 => -1,
    _ => 0,
  };

  sign * order + seconds / HALF_LIFE
}
