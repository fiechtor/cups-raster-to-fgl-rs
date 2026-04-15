# cups-raster-to-fgl-rs

> [!NOTE]  
> Fair warning, this project was slopped together using OpenCode with Qwen3 Code Next FP8 and Grok. It works for limited purposes. Please do not report any issues, I will not address them.

Converts CUPS Raster v3 format to FGL (Friendly Ghost Language) printer commands.

Based on the work of [pretix/cups-fgl-printers](https://github.com/pretix/cups-fgl-printers/).

## Overview

This tool reads CUPS Raster v3 format from stdin or a file and outputs FGL printer commands to stdout. It's designed to work as a CUPS filter for BOCA TLS Lemur printers, which use the FGL language. Potentially also works for other printers that use this language.

Built as a debugging tool to assess some issues I had with the BOCA TLS Lemur C on my linux machine, a preview mode has been implemented to convert the FGL Graphics Mode commands to readable images. 

## Usage

### Basic Conversion (stdin)

```bash
# Read from stdin, output FGL to stdout (CUPS standard)
cat input.ras3 | cups-raster-to-fgl-rs

# Or pipe directly from CUPS
cups-raster-to-fgl-rs < input.ras3 > output.fgl
```

### File Input

```bash
# Read from file, output FGL to stdout
cups-raster-to-fgl-rs input.ras3 > output.fgl
```

### Preview Mode

Generate a PNG preview of what will be printed:

```bash
# Preview with processed (binary) image
cups-raster-to-fgl-rs --preview input.ras3
# Creates: input_processed.png

# Preview with raw (grayscale) pixels
cups-raster-to-fgl-rs --preview --preview-raw input.ras3
# Creates: input_raw.png

# Preview FGL file directly (skips conversion)
cups-raster-to-fgl-rs --preview input.fgl
# Creates: input_fgl.png
```

## FGL Preview

The preview feature can also render FGL files directly without conversion:

- Auto-detects input format (RAS3 or FGL)
- Parses FGL commands (`<RCy,x>`, `<Gcount>`, `<p>`, `<q>`, `<CB>`)
- Renders binary black/white image
- Each logical pixel scaled 2x2 for visibility

## Preview Files

When `--preview` is used:
- **RAS3 input**: Creates `{input}_raw.png` or `{input}_processed.png`
- **FGL input**: Creates `{input}_fgl.png`
- **Stdin**: Creates `preview_raw.png`, `preview_processed.png`, or `preview_fgl.png`

For multi-page RAS3 files, pages are numbered: `{input}_raw1.png`, `{input}_raw2.png`, etc.

## FGL Command Reference

The output uses these FGL commands:

- `<RCy,x>` - Set cursor position to row y, column x
- `<Gcount>` - Send `count` bytes of graphics data (each byte = 1 column of 8 vertical dots)
- `<p>` - Print job and cut
- `<q>` - Print job without cut
- `<CB>` - Clear buffer

### Graphics Data Format

Each byte represents one column of 8 dots:
- Bit 7 (MSB) = top dot (row + 0)
- Bit 0 (LSB) = bottom dot (row + 7)
- 1 = black dot, 0 = blank

## Requirements

- Rust 1.70+
- `image` crate for PNG generation

## Building

```bash
cargo build --release
```

## License

MIT
