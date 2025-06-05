# knitlang

*the language for knitters/crocheters by a crocheter/knitter.*

> [!IMPORTANT]
> This language is still in active development. Please use caution as it's still *really* buggy.

## What Is knitlang?

knitlang was made in June 2025, by me, [kenny](https://github.com/nonbinarybyte). Originally it was supposed to be a rust remake of binary called "bin+" (`.binp` as a file extension.) However I decided to change that idea up after i realized 2 things...

1. Rebuilding `1001110`'s just isnt possible and/or would be too hard.
2. Im new to rust.

**SO Then...!**

a brilliant person in discord.gg/program came up with an idea (mostly as a joke) that i should make something based off of knitting. So i did. (*mostly* for shits and giggles)

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section --> 
<table> <tr> <td align="center"><a href="https://github.com/nonbinarybyte"><img src="https://avatars.githubusercontent.com/u/184435682?v=4" width="100px;" alt="Kenny Thomas-Moore"/><br /><sub><b>Kenny Thomas-Moore</b></sub></a><br /><a href="#code-nonbinarybyte" title="Code">💻</a> <a href="#doc-nonbinarybyte" title="Docs">📖</a> <a href="#translation-nonbinarybyte" title="Translation">🌍</a> <a href="#ally-nonbinarybyte" title="Allyship">🏳️‍🌈</a> <a href="#maintenance-nonbinarybyte" title="Maintenance">🚧</a></td> <td align="center"><a href="https://github.com/johndavedosn"><img src="https://avatars.githubusercontent.com/u/112868391?v=4" width="100px;" alt="Evil"/><br /><sub><b>Evil</b></sub></a><br /><a href="#code-johndavedosn" title="Code">💻</a> <a href="#maintenance-johndavedosn" title="Maintenance">🚧</a></td> <td align="center"><a href="https://github.com/mnem42"><img src="https://avatars.githubusercontent.com/u/177770058?v=4" width="100px;" alt="mnem"/><br /><sub><b>mnem</b></sub></a><br /><a href="#code-mnem42" title="Code">💻</a> <a href="#maintenance-mnem42" title="Maintenance">🚧</a></td> </tr> </table>
<!-- ALL-CONTRIBUTORS-LIST:END -->

## How do I use knitlang?

simply build from source using the following commands...

```bash
cargo build --release
./target/release/knitlang pattern.knit
```

> [!NOTE]
> `.knit` is our custom file extension, however for `.knit` files to work you *may* need to have the source in the root directory of your project.

### Syntax Glossary

| 🧵 Knitting Term            | 💻 Programming Equivalent         | 💬 Notes                                                                     |
| --------------------------- | --------------------------------- | ---------------------------------------------------------------------------- |
| `K` (knit)                  | Operation / Function              | Core unit of work; executes a line or step                                   |
| `P` (purl)                  | Variable declaration              | Stores yarn, needle info, row state                                          |
| `YO` (yarn over)            | Increment / Loop initialization   | Adds a new stitch (increases count)                                          |
| `K2tog` (knit 2 together)   | If/Else or Conditional Logic      | Combines stitches → test & react (compression or decision)                   |
| `SSK` (slip, slip, knit)    | Comparison / Merge                | Rearranges or compares values                                                |
| `Pattern repeat`            | Loops (for/while)                 | Repeat blocks of instructions/stitches                                       |
| `* * repeat n times`        | `for` loop                        | Repeats a pattern block a specific number of times                           |
| `Cast on`                   | Program start / Init block        | Beginning of execution; memory allocation                                    |
| `Bind off`                  | Program end / Return / Cleanup    | Frees memory / closes output                                                 |
| `Stitch marker`             | Label / Anchor / Checkpoint       | Used to mark positions in loops or branches                                  |
| `Gauge`                     | Type checking / Size constraints  | Ensures consistent output (e.g., size, speed, type)                          |
| `Row` / `Round`             | Code block / Function call        | A complete unit of execution; stack frame analog                             |
| `Increase`                  | Push to list / Expand memory      | Adds a new element                                                           |
| `Decrease`                  | Pop from list / Reduce memory     | Removes an element                                                           |
| `Tension`                   | Runtime performance tuning        | Balancing tightness/efficiency                                               |
| `Frogging`                  | Rollback / Undo / Debug           | Rip it out and try again 🐸                                                  |
| `Stitch holder`             | Temporary variable / Cache        | Stores values for reuse or reassignment                                      |
| `Cable`                     | Thread switching / Threaded logic | Rearranges stitch execution order (like parallelism or non-linear execution) |
| `Chart` / `Written pattern` | Source code or DSL file           | Human-readable source                                                        |
| `Swatch`                    | Test / Compile                    | Prototype run of a block before full program                                 |
