pub const HTML_START: &str = r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
body{
  background-color: lavender;
}
.prettyprint ol.linenums > li {
	list-style-type: decimal;
}
.wrap{
  display: flex;
  margin: 0 auto;
}
main{
}
.nav{
  width: 180px;
}
.pair{
  padding: 1rem;
  border-radius: 1rem;
  border-bottom: solid 3px #000;
}
h3{
  border-bottom: solid 2px #000;
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
