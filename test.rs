extern crate time;

use super::rank;

#[test]
fn should_rank_stuff () {
  let epoch = time::now_utc();

  let timestamp = time::now_utc();
  let upvotes: i32 = 3;
  let downvotes: i32 = 1;
  let ranking = rank(&upvotes, &downvotes, timestamp, epoch);
  assert_eq!(ranking > 1, true);
}
