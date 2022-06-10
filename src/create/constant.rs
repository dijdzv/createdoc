pub const HTML_START: &str = r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
.prettyprint ol.linenums > li {
	list-style-type: decimal;
}
.wrap{
  display: flex;
}
main{
  width: 80%;
}
.nav{
  width: 180px;
  border: solid 1px #000;
}
.pair{
  background-color: lavender;
  margin-bottom: 3rem;
  padding: 1rem;
  border-radius: 1rem;
}
p{
  margin-left: 2rem;
  font-size: 1rem;
}
code {
  font-size: 1rem;
  padding: 0.1em 0.25em;
}

</style>
<title></title>
</head>
<body>
"#;

pub const HTML_END: &str = r#"
<script src="https://cdn.jsdelivr.net/gh/google/code-prettify@master/loader/run_prettify.js?lang=php&skin=sunburst"></script>
</body>
</html>"#;

pub const TRIM_PATTERN: [char; 3] = ['/', '*', ' '];
