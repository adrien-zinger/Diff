# Diff

Compute the diff between two vectors with the [Levenshtein](https://en.wikipedia.org/wiki/Levenshtein_distance) algorithm.

In order to continue my data sharing P2P project from scratch I want to study data compression and diff compution. :zap:

## Roadmap
- [x] Create a *diff* file between two binary files *src* *target*
- [x] Compute a *dest* file combining *target* + *diff*
- [x] Restore old target file with *dest* - *diff*
- [ ] create a real usable tool to get diffs recursively in a folder tree
- [ ] Think about optimising data compression
- [ ] Make unit tests testing something instead of verify manually with `cargo test -- --nocapture`