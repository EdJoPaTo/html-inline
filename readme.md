Check out https://github.com/makovich/inliners


# HTML Image Inline base64

> Reads an html file and inlines all the images with base64

This allows for html files which are self contained.
For whatever reason no Markdown to HTML tool seems to be be able to do so.

Replaces

```html
<img src="media/image.png">
```

into

```html
<img src="data:image/png;base64,iVBORw0K…">
```


## Usage

```plaintext
HTML Image Inline base64 0.1.0
EdJoPaTo <html-image-inline-base64-rust@edjopato.de>
Reads an html file and inlines all the images with base64

USAGE:
    html-image-inline-base64 [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>    Path to output the final html

ARGS:
    <FILE>    Path to html file
```
