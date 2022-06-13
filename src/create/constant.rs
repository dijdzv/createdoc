pub const HTML_START: &str = r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
html {
  scroll-behavior: smooth;
}
*{
  box-sizing: border-box;
}
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
  height: calc(100vh - (65px + 2rem));
  overflow-y: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;
}
.n-folder::-webkit-scrollbar {
  display: none;
}
.n-file:hover{
  background-color: #a4a8d433;
}
.n-filename{
  cursor: pointer;
  margin: 0;
  padding-left: 0.5rem;
  color: #633f86;
  border-radius: 0.3rem;
}
.n-filename:hover{
  background-color: #a4a8d455;
}
.n-func{
  list-style: none;
  padding: 0;
  margin: 0;
}
.n-func li{
  color: #734f96;
  font-size: 0.95rem;
  padding-left: 1rem;
  margin-top: 0.2rem;
  padding-left: 0.5rem;
  border-radius: 0.3rem;
}
.n-func li:hover{
  background-color: #a4a8d455;
}
.dn{
  display: none;
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
  width: min(800px,calc(100vw - 200px));
}
h2{
  margin: 1rem 0 0 1rem;
  padding-left: 0.5rem;
  color: #633f86;
  border-bottom: solid 3px #633f86;
}
.pair{
  margin-left: 1rem;
  padding: 1rem;
  border-radius: 1rem;
}
.m-func_name{
  position: relative;
  margin: 0;
  padding-top: 1rem;
  padding-left: 2rem;
  color: #734f96;
}
.m-func_name::before{
  content: "";
  position: absolute;
  top: 1rem;
  bottom: 0;
  left: 0;
  right: 0;
  border-top: solid 0px #734f96;
  border-bottom: solid 2px #734f96;
  border-left: solid 20px #734f96;
  border-right: solid 0px #734f96;
  border-radius: 0px;
}
p{
  line-height: 1.2rem;
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
let nFile = document.getElementsByClassName("n-filename");

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
