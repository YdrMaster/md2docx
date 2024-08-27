# *docx* builder

这个工具用于阅读文本格式并生成适用于 Microsoft Word 的 docx 文档。

## 使用说明

```plaintext 使用说明命令
cargo util-help
```

```plaintext 使用说明
Usage: md2docx.exe <COMMAND>

Commands:
  show
  convert
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 解析文本格式

解析并展示文本格式内容，用于检测支持性。

```plaintext 解析文本命令
cargo show --help
```

```plaintext 解析文本使用说明
Usage: md2docx.exe show <FILE>

Arguments:
  <FILE>  The file to show

Options:
  -h, --help  Print help
```

### 转换到 docx

```plaintext 转换命令
cargo convert --help
```

```plaintext 转换使用说明
Usage: md2docx.exe convert [OPTIONS] <FILE>

Arguments:
  <FILE>  The file to convert

Options:
  -s, --style <STYLE>  A Toml style file
  -h, --help           Print help
```

示例样式文件：

```toml 示例样式文件
# 标题
[Heading1]
font = "黑体"
font-size = "二号"
align = "center"

[Heading2]
font = "黑体"
font-size = "小二"
align = "left"

# 正文
[BodyText]
font = "仿宋"
font-size = "三号"

# 题注
[Caption]
font = "黑体"
font-size = "四号"

# 代码
[Code]
font = "Cascadia Code Light"
font-size = "四号"
```

## 依赖项

*[markdown-1.0](https://crates.io/crates/markdown/1.0.0-alpha.20)*

## 参考资料

- [多级列表格式](https://learn.microsoft.com/zh-cn/dotnet/api/documentformat.openxml.wordprocessing.numberformatvalues)

## 测试结构

### --- Heading 3

A **B `CD` *~E~ **F`G`** H* I** ~***J***~

- **Unordered**List 1
- **Unordered**List 2
  Text in list.

1. *List 1*

   Text in list.

   Text in list.
2. List *2*

```markdown This is markdown
# h1
## h2

text
```

> Quote line

> Quote paragraph 1
>
> Quote paragraph 2
>
> > Quote Quote
>
> 1. Quote list 1
> 2. Quote list 2
>
> ```rust This is quote code
> let rust = "rust";
> ```

> **NOTICE** This is a table ↓

Title 1 | Title 2 | Title 3
--------|--------:|:-------:
Item1:1 | Item1:2 | Item1:3
Item2:1 | Item2:2 | Item2:3
Item3:1 | Item3:2 | Item3:3

![logo](md.png)
