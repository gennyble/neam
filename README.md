# neam
Neam is a small command line tool to scale PNGs using Nearest Neighboor. I encourage you to look at the code, there isn't much going on.

Give neam a size and the input file (only PNG for now) and you'll get an output in the same place as the input. It'll have `_widthxheight` appended to the end of the file name.

For example, `neam -s 512x512 ~/images/image.png` will produce `~/images/image_512x512.png`, overwriting the file with the same name, if it exists. You may also use a comma to seperate the width and height if you prefer, like this: `-s 512,512`

### License
This software is licensed under the Anti-Capitalist Software License. You can read the license text in the LICENSE file in the repositories root, or on it's website: <https://anticapitalist.software>