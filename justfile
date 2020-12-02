new-day:
    #! /usr/bin/env python
    import os

    inputs = os.listdir('./inputs')
    new_day = sorted(inputs)[-1][3:5]

    # add entry to src/main.rs days_handlers
    # add mod day## to src/days/mod.rs
    # add pub use day##::day## to src/days/mod.rs

    with open(f'src/days/day{new_day}.rs', 'w') as f:
        f.write('use crate::Solution;\n')
        f.write('\n')
        f.write('pub fn day' + new_day + '(input: &str) -> Solution<u32> {\n')
        f.write('    Solution { part1: 0, part2: 0 }\n')
        f.write('}\n')
        f.write('\n')
        f.write('#[cfg(test)]\n')
        f.write('mod tests {\n')
        f.write('    use super::*;\n')
        f.write('\n')
        f.write('    #[test]\n')
        f.write('    fn test_day' + new_day + '() {}\n')
        f.write('}\n')
