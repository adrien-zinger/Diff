# Diff
Compute the diff between two vectors with the [Levenshtein](https://en.wikipedia.org/wiki/Levenshtein_distance) algorithm.

In order to continue my data sharing P2P project from scratch I want to study data compression and diff compution. :zap:
# Usage
Normal use for binaries:
- `diff_tool first.pack second.pack mydiff.d`
- `diff_tool --apply=true mydiff.d first.pack dest.pack`

Normal use for plain text files:
- `diff_tool first.txt second.txt mydiff.d`
- `diff_tool --binary=false --apply=true mydiff.d first.txt dest.txt`

## Roadmap
- [x] Create a *diff* file between two binary files *src* *target*
- [x] Compute a *dest* file combining *target* + *diff*
- [x] Restore old target file with *dest* - *diff*
- [x] create a real usable tool to get diffs recursively in a folder tree
- [ ] Think about optimising data compression
- [ ] Make unit tests testing something instead of verify manually with `cargo test -- --nocapture`