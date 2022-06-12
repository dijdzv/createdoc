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
.n-folder{
  position: fixed;
  padding: 0;
  margin: 0;
  width: 200px;
  height: 90%;
  overflow-y: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;
}
.n-folder::-webkit-scrollbar {
  display: none;
}
.n-file{
  font-size: 1.4rem;
}
.n-file:hover{
  background-color: #a4a8d4aa;
}
.n-func li{
  font-size: 0.95rem;
  padding-left: 1rem;
}
.n-func li:hover{
  background-color: #a4a8d455;
}
.dn{
  display: none;
}
ul{
  list-style: none;
}
li{
  margin-top: 0.2rem;
  padding-left: 0.5rem;
  border-radius: 0.3rem;
}
footer{
  position: fixed;
  width: calc(200px - 1rem);
  bottom: 1rem;
  padding-top: 1rem;
  padding-left: 1rem;
  border-top: solid 1px #734f9655;
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
</style>
<title></title>
</head>
<body>
"#;

pub const HTML_END: &str = r#"
<script src="https://cdn.jsdelivr.net/gh/google/code-prettify@master/loader/run_prettify.js?lang=php&skin=sunburst"></script>
<script>
let nFile = document.getElementsByClassName("n-file");

for (let f of nFile) {
  f.addEventListener("click", () => {
    let id = f.getAttribute("id");
    let cls = document.getElementsByClassName(id)[0];
    cls.classList.toggle("dn");
  });
}


</script>
</body>
</html>"#;

pub const TRIM_PATTERN: [char; 3] = ['/', '*', ' '];
