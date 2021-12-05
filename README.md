# neam
Neam is a small command line tool to scale PNGs using Nearest Neighboor. I encourage you to look at the code, there isn't much going on.

Neam wants an image, a scale, and an optional output path. If you don't give it an input path the output file will be placed in the same directory as the input. The filename will be the same except an underscore and then the scale will be added. The scale can be a width/height: 512x512, or it can be a percent: 200%.

```
Usage: neam input_file $scale [output_file]

$scale:
	Scale can be a width and height separated by an x, WidthxHeight, or
	it can be a percent to scale by: 14%, 50%, 200%, etc.

output_file:
	If no output path is provided, it defaults to the input
	file name with the scale appended after an underscore.
		original.png 200%    -> original_2.0x.png
		original.png 512x512 -> original_512x512.png
```

### License
This software is licensed under the Anti-Capitalist Software License. You can read the license text in the LICENSE file in the repositories root, or on it's website: <https://anticapitalist.software>