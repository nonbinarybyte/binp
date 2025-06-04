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

## How do I use knitlang?

simply build from source using the following commands...

```bash
cargo build --release
./target/release/knitlang pattern.knit
```

> [!NOTE]
> `.knit` is our custom file extension, however for `.knit` files to work you *may* need to have the source in the root directory of your project.

- `K` = operation/function
- `P` = variable declaration
- `YO` = increment/loop
- `K2tog` = If/Else / Logic Branch
- `Pattern repeat` = loop(s)
- `Cast on` = Start/Init block
- `Bind off` = End/Return
