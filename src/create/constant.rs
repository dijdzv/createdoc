pub const HTML_START: &str = r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
body{
  background-color: #e6e6fa88;
}
.prettyprint ol.linenums > li {
	list-style-type: decimal;
}
.wrap{
  width: fit-content;
  display: flex;
  margin: 0 auto;
}
nav{
  width: 200px;
  border-right: solid 1px #734f9633;
  background-color: #e6e6faee;
}
.file li{
  border-radius: 0.3rem;
}
.file > li:hover{
  background-color: #a4a8d4aa;
}
.func li{
  padding-left: 1rem;
  border-radius: 0.3rem;
}
.func li:hover{
  background-color: #a4a8d455;
}
.display{
  display: none;
}
ul{
  list-style: none;
  position: fixed;
  padding: 0;
  margin: 0;
  width: 200px;
  /* height: 95vh; */
  overflow-y: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;
}
ul::-webkit-scrollbar {
  display: none;
}
li{
  font-size: 0.95rem;
  margin-top: 0.2rem;
  padding-left: 0.5rem;
}
main{
  width: 800px;
}
.pair{
  padding: 1rem;
  border-radius: 1rem;
}
h3{
  color: #734f96;
  border-bottom: solid 2px #734f96;
  padding-top: 1rem;
}
p{
  margin-left: 2rem;
  font-size: 1rem;
}
code{
  font-size: 1rem;
}
a{
  color: inherit;
  text-decoration: none;
}
footer{
  position: fixed;
  width: calc(200px - 1rem);
  bottom: 1rem;
  padding-top: 1rem;
  padding-left: 1rem;
  border-top: solid 1px #734f9655;
}
</style>
<title></title>
</head>
<body>
"#;

pub const HTML_END: &str = r#"
<script src="https://cdn.jsdelivr.net/gh/google/code-prettify@master/loader/run_prettify.js?lang=php&skin=sunburst"></script>
<script>
let file = document.getElementById("csv");
let func = document.getElementsByClassName("csv")[0];
csv.onclick = () => {
    func.classList.toggle("display");
};
</script>
</body>
</html>"#;

pub const TRIM_PATTERN: [char; 3] = ['/', '*', ' '];
