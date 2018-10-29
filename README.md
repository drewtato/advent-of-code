# Advent of Code 2017

Hello, this is drewtato coming from almost Halloween 2018, aka [alternate Christmas](https://en.wikipedia.org/wiki/Mathematical_joke#Jokes_with_numeral_bases), having finished Advent of Code 2017. It took me about 10 months too long, but it is done! There's some slightly elegant algorithms here, and some nasty, hacky, uncommented messes as well.

I started this a few days after the beginning, and while I did not get any actual leaderboard points, I did finish under 1000th place for seven stars (days 9, 13, 14, and first part of 16) with a lowest on day 13 part 2 of 388th place in 44 minutes. Full stats:

```text
      --------Part 1--------   --------Part 2--------
Day       Time   Rank  Score       Time   Rank  Score
 25       >24h   5100      0       >24h   4347      0
 24       >24h   4929      0       >24h   4861      0
 23       >24h   5697      0       >24h   4884      0
 22       >24h   5664      0       >24h   5533      0
 21       >24h   5341      0       >24h   5267      0
 20       >24h   5373      0       >24h   6434      0
 19       >24h   5799      0       >24h   5726      0
 18   01:59:49   1257      0       >24h   6760      0
 17   05:14:49   2519      0   05:21:35   2062      0
 16   00:47:11    863      0       >24h   8491      0
 15   02:17:42   1758      0   03:23:07   2035      0
 14   00:51:17    971      0   01:25:35    691      0
 13   00:20:45    397      0   00:44:03    388      0
 12   02:25:41   2110      0   03:55:47   2554      0
 11   05:16:10   2910      0   05:19:35   2701      0
 10       >24h   6622      0       >24h   5934      0
  9   00:36:45    839      0   00:40:07    786      0
  8       >24h   8880      0       >24h   8699      0
  7   01:31:38   2150      0       >24h   9309      0
  6   05:10:18   4824      0   05:14:43   4596      0
  5   07:15:08   6909      0       >24h  15903      0
  4       >24h  13735      0       >24h  17513      0
  3       >24h  15712      0       >24h  15220      0
  2       >24h  23814      0       >24h  22562      0
  1       >24h  29954      0       >24h  27170      0
  ```

Looking at the stats now, I see a few people have finished the last few after me. Good luck!

It has been a goal of mine since AoC 2016 to finish all of it. I didn't get even close for 2016, and when 2017 came I tried again, and I finally did it!

Day 10 was annoying because I didn't realize I wasn't zero-padding the hexadecimal. It took way too long and I had to look for help on Reddit for that one.

I did day 13 part 1 in [Factorio](https://www.factorio.com) (a video game, not a programming language) as well, which looked like this:

![Day 13 in Factorio](https://i.imgur.com/MSXGpKn.jpg)

If you have the game and want to try it, here's the [blueprint string](https://pastebin.com/gLGLV5aj). Going from left to right: a combinator that counts from zero, acting as a for loop through the layers. The input is in constant combinators in number of iron and copper. The next combinator lets the previous combinator's signal through when the counter reaches its index. The last long column assigns indexes to the previous one.

The signal for a single iteration is a number of iron, representing depth, and a number of copper, representing range. The first few check if the packet was caught, and if so, it lets range * depth through, and the last combinator accumulates all the severities. The lamps display this (here we have the answer 2384).

Something to note with this is that it uses very aggressive pipelining. The counter counts up every tick, which means a new signal is sent into the computation part every tick. The iteration being processed at the first combinator is four iterations after the iteration being at the end of the computation part. Factorio is pretty good with this, since if the signal was sent to the accumulator for two ticks, it would count it twice. This is also within four ticks of being the theoretical fastest solution that processes each input line individually.

## This Repo

Originally I had a bunch of .rs files at the top level, but that got messy so I made them all their own cargo init directory, with the usual cargo structure, plus my input file titled with the day number. You can try them all with `cargo run`, and they should work on the latest Rust (there isn't a whole lot of weird stuff), with the last ones definitely working on Rust 1.31.0 nightly.

### Output

Most of the days output two lines: the part 1 answer and the part 2 answer. Some of them just output part 2, and some output something else; it depended on how I felt and how fast I wanted to finish the problem.

## Regrets

If I did 2018 in Rust again (not likely), I would make each day a library or module (not sure how these each affect compile—and therefore testing and challenge completion—speed, so I would have to test first) and run each one with arguments passed to the main. That way I could more easily time the whole thing and [RLS](https://github.com/rust-lang-nursery/rls) would actually work (it doesn't for this one as-is because the top level isn't a cargo directory).

*T.Y. Eric, and Happy coding!*