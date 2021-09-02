# Diff

Compute the diff between two vectors with the [Levenshtein](https://en.wikipedia.org/wiki/Levenshtein_distance) algorithm.

In order to continue my data sharing P2P project from scratch I want to study data compression and diff compution. :zap:

## Roadmap
- [x] Create a *diff* file between two binary files *src* *target*
- [ ] Compute a *dest* file combining *target* + *diff*
- [ ] Restore old target file with *dest* - *diff*
- [ ] Think about optimising data compression
- [ ] create a real usable tool to get diffs recursively in a folder tree