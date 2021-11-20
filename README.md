# neam
Neam is a small command line tool to scale PNGs using Nearest Neighboor. I encourage you to look at the code, there isn't much going on.

Give neam a size (`-s`), the input file (first free argument), and an output file (`-o`). If you don't specify an output file, the scaled image will be in the same directory as the input with `_widthxheight` appended to the name. If your output file exists already it will be overwritten.

For example, `neam -s 512x512 ~/images/image.png` will produce `~/images/image_512x512.png`. You may also use a comma to seperate the width and height if you prefer. Like this: `-s 512,512`

```
usage: neam FILE [options]

Options:
    -s, --size SIZE     The new size of the image.
                        You can separate width/height with an x or a comma.
						Ex: 512x512 or 512,512
    -o, --output PATH   The name of the output file.
                        Defaults to the input name with _widthxheight appended.
    -h, --help          Print this help message
```

### License
This software is licensed under the Anti-Capitalist Software License. You can read the license text in the LICENSE file in the repositories root, or on it's website: <https://anticapitalist.software>