# File-To-Chess

Simple Rust program to convert any file to a chess game(s).

## ChessEncryption

A LOT of this code is inspired/stolen from [WintrCat/chessencryption](https://github.com/WintrCat/chessencryption) so make sure to look at it.

## Encoding file to a chess games

```bash
filetochess -e image.jpg
```

This command will save the image.jpg in folder called `games`.

## Decoding file from a chess games

```bash
filetochess -d output_file.jpg
```

You need to run this command in the same folder as the generated `games` folder is in.

# Note!

There are probably gonna be a lot of issues so feel free to create PR or Issue if you find any.
