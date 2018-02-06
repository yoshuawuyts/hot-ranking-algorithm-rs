extern crate chrono;

pub use chrono::offset::Utc;
pub use chrono::DateTime;
use std::cmp;

const HALF_LIFE: i32 = 45_000;

/// Calculate the relevancy of an item based on upvotes, downvotes, a current
/// timestamp and the epoch at which the program is first run (e.g. no value can
/// be lower than that).
pub fn rank(
  upvotes: &i32,
  downvotes: &i32,
  timestamp: DateTime<Utc>,
  epoch: DateTime<Utc>,
) -> i32 {
  let duration = timestamp.signed_duration_since(epoch);
  let seconds = duration.num_seconds() as i32;

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

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::prelude::*;

  #[test]
  fn one_upvote_one_downvote() {
    let upvotes: i32 = 1;
    let downvotes: i32 = 1;
    let epoch = Utc.ymd(2014, 1, 1).and_hms(21, 15, 33);
    let timestamp = Utc.ymd(2014, 1, 1).and_hms(21, 15, 33);

    let ranking = rank(&upvotes, &downvotes, timestamp, epoch);
    assert!(ranking == 0);
  }

  #[test]
  fn twenty_upvotes_zero_downvotes() {
    let upvotes: i32 = 50;
    let downvotes: i32 = 0;
    let epoch = Utc.ymd(2014, 1, 1).and_hms(21, 15, 33);
    let timestamp = Utc.ymd(2014, 1, 1).and_hms(21, 15, 33);

    let ranking = rank(&upvotes, &downvotes, timestamp, epoch);
    assert!(ranking > 0);
  }

  #[test]
  fn one_upvote_twenty_downvotes() {
    let upvotes: i32 = 1;
    let downvotes: i32 = 1000;
    let epoch = Utc.ymd(2014, 1, 1).and_hms(21, 15, 33);
    let timestamp = Utc.ymd(2014, 1, 1).and_hms(21, 15, 33);

    let ranking = rank(&upvotes, &downvotes, timestamp, epoch);
    println!("{}", ranking);
    assert!(ranking == 0);
  }
}
