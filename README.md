# boggler

A command line boggle board solver for n-dimensional boggle boards.

## Installation

```shell
git clone git@github.com/mcountryman/boggler.git
cargo build
cp target/release/boggler /usr/local/bin/boggler
```

## Example

```shell
Finished release [optimized] target(s) in 0.03s
  Running `target/release/boggler --grid txrasomeyiftooetmtnglodnf`
aeon, aery, are, arm, armet, asio, ayr, dol, dolt, dot, ear, eas, easy, ego, emf, end, eon, era, etf, eyas, eye, eyra,
eyre, fox, fto, gen, genf, genome, gent, gnome, goer, gond, gone, goo, gooey, goon, gooney, goy, goya, ion, iyar, lot,
lotte, lottery, lotto, ltm, ltte, mod, mol, molt, mom, momot, mon, money, moneyer, mongo, mono, moo, moon, mooneye, moong,
moot, mot, mote, motmot, mott, motto, neo, ney, ngf, ngo, nne, nog, noisy, nome, not, note, oft, ois, olm, one, ono, oto,
otoe, otter, oxeye, ras, ray, rayon, rem, ret, rex, rya, rye, say, sion, tea, tear, teary, tera, teras, term, text, tnf,
tod, toe, toea, tog, tom, tome, ton, tone, too, toon, toot, tot, tote, totem, toter, totter, tottery, toy, toyon, yea,
year, yen, yer, yet, yon,

len 3: 64
len 4: 38
len 5: 19
len 6: 4
len 7: 4
total: 129
done in 70.7273ms
```

## Usage

```shell
boggler [OPTIONS] --grid <GRID>
```

## Options

```
-d, --dictionary <DICTIONARY>
        The optional path to a text file containing words delimited by newline characters.

        Default: Uses dictionary baked into executable.
-g, --grid <GRID>
        The characters in the grid.

        Example: "modnstedetripyij"
-h, --help
        Print help information
-m, --min-word-len <MIN_WORD_LEN>
        The shortest word length to match

        [default: 3]
-n, --neighbors-kernel <NEIGHBORS_KERNEL>
        A space separated list of comma separated tuples to determine which neighboring cells to
        use when checking a cell.

        Defaults to all neighbors.

        [default: "-1,-1 0,-1 -1,0 1,1 0,1 1,0 -1,1 1,-1"]
-V, --version
        Print version information
```
