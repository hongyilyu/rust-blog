---
title: Extended Sample Markdown File for Tailwind CSS Typography Testing
publication_date: 2023-12-31
description: Testing the layout of the site.
reading_time: 10 min
---



```rust
fn visit_dirs(dir: &PathBuf, cb: &mut dyn FnMut(PathBuf)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(path);
            }
        }
    }
    Ok(())
}
```



This extended sample Markdown file is created to test the Tailwind CSS typography. It includes a wider range of Markdown elements like tables, footnotes, strikethroughs, and task lists.

### Paragraphs
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

#### Strikethrough
This is an example of ~~strikethrough~~ text.

### Lists
- Item 1
- Item 2
  - Subitem 2.1
  - Subitem 2.2
- Item 3

#### Numbered List
1. First item
2. Second item
3. Third item

##### Task List
- [x] Completed task
- [ ] Incomplete task
- [ ] Another incomplete task

###### Blockquotes
> This is a blockquote. It is used to indicate quoted text.

## Tables
| Heading 1 | Heading 2 | Heading 3 |
|-----------|-----------|-----------|
| Row 1 Col 1 | Row 1 Col 2 | Row 1 Col 3 |
| Row 2 Col 1 | Row 2 Col 2 | Row 2 Col 3 |
| Row 3 Col 1 | Row 3 Col 2 | Row 3 Col 3 |

## Code Blocks
Inline `code` has `back-ticks around` it.

```javascript
// JavaScript code block
function helloWorld() {
  console.log("Hello, world!");
}
```

### Links and Footnotes
This is a [link to Tailwind CSS](https://tailwindcss.com) website.
Here's a simple footnote[^1].

[^1]: This is the footnote text.

### Images
![Alt text for the image](https://via.placeholder.com/150)
![test for hack](./rust.jpeg)
![test for hack2](https://github.com/hongyilyu/rust-blog/blob/master/posts/blogs/2023/sample-markdown-test/rust.jpeg?raw=true)
## Conclusion
This extended Markdown file should help in testing various typographic elements like tables, footnotes, strikethroughs, and task lists, styled using Tailwind CSS.
