# HTML Inline

> Reads an HTML file and inlines all the images and stylesheets

This allows for HTML files which are self-contained.

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
HTML Inline 0.3.0
EdJoPaTo <html-inline-rust@edjopato.de>
Reads an HTML file and inlines all the images and stylesheets

USAGE:
    html-inline [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>    Path to output the final html

ARGS:
    <FILE>    Path to html file
```

## Alternatives

- https://github.com/Y2Z/monolith
- https://github.com/8176135/inline-assets-rs
- https://github.com/makovich/inliners
