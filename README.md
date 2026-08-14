# C Blunt

Brainfuck, but you can't C Sharp — so you're C-ing blunt. Source code is all block characters (`▀▄█░▒▓`) like everything's blurred out. Transpiles `.cb` files to C, or to C# if you put on glasses.   My Magnum Opus.

## The Language

C Blunt is Brainfuck with a visual gag: operators are ASCII blocks instead of `><+-.,[]`.

| Character | Brainfuck | Meaning |
|-----------|-----------|---------|
| `▌` | `>` | Move pointer right |
| `▐` | `<` | Move pointer left |
| `▀` | `+` | Increment cell |
| `▄` | `-` | Decrement cell |
| `█` | `.` | Output character |
| `░` | `,` | Input character |
| `▒` | `[` | Loop while cell ≠ 0 |
| `▓` | `]` | End loop |

Anything else is a comment. Tape size is 30,000 cells (standard Brainfuck).

## Build

```bash
cargo build --release
```

## Usage

### Transpile to C (default — still blunt)

```bash
c_blunt examples/hello.cb
```

This writes `examples/hello.c`. Compile it yourself to get a native binary:

```bash
cc -O3 examples/hello.c -o hello
./hello
```

Custom output path:

```bash
c_blunt examples/hello.cb -o my_program.c
```

### Put on glasses (transpile to C#)

```bash
c_blunt glasses examples/hello.cb
```

This writes `examples/hello.cs` — run it however you like.

Custom output:

```bash
c_blunt glasses examples/hello.cb -o hello.cs
dotnet run hello.cs
```

👓 Glasses mode lets you actually C Sharp.

## Example

`examples/hello.cb` prints `Hello World!`:

```
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█▀▀▀▀▀▀▀██▀▀▀█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█▀▀▀█▄▄▄▄▄▄█▄▄▄▄▄▄▄▄█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█
```

Looks like a solid wall of blocks. Runs like normal Brainfuck.

## Help

```bash
c_blunt help
```

---

*can't C Sharp? C Blunt.*

*the dotnet team has been real quiet since this dropped*
