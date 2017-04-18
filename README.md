# hot-ranking-algorithm-rs
Algorithm that measures how relevant a given data set is, kinda like Reddit.

Doesn't do any jitter stuff and can probably be gamed if you try hard enough,
but it should be useful enough if you want to wire up community-driven news
site that always has fresh, relevant content for people to discover.

## Usage
```rust
extern crate hot_ranking_algorithm;
extern crate time;

use hot-ranking-algorithm::rank;

let epoch = time::now_utc();
let timestamp = time::now_utc();
let upvotes = 3;
let downvotes = 1;
let ranking = rank(upvotes, downvotes, timestamp, epoch);
println!('{}', ranking);
// => 2
```

## API
### `rank(upvotes: &i32, downvotes: &i32, timestamp: &SteadyTime, epoch: &SteadyTime) -> result: i32`
Calculate the relevancy of an item based on upvotes, downvotes, a current
timestamp and the epoch at which the program is first run (e.g. no value can be
lower than that).

## Installation
```sh
$ cargo add hot-ranking-algorithm
```

## See Also
- https://github.com/yoshuawuyts/pretty-hot-ranking-algorithm

## License
[MIT](https://tldrlegal.com/license/mit-license)
