pub const STYLE: &str = r#"<style>
html {
  scroll-behavior: smooth;
}
*{
  box-sizing: border-box;
}
body{
  background-color: #e6e6fa88;
  overflow-x: hidden;
  margin: 0;
}
.wrap{
  display: flex;
}
nav{
  position: sticky;
  top: 0;
  left: 0;
  width: 250px;
  min-width: 200px;
  height: 100vh;
  border-right: solid 1px #734f9633;
  background-color: #e6e6faee;
}
.html-filename{
  position: sticky;
  top: 0;
  margin: 0;
  padding: 0;
  height: 4rem;
  line-height: 4rem;
  text-align: center;
  color: #633f86;
  border-bottom: solid 1px #734f9655;
  overflow: hidden;
}
.n-folder{
  position: sticky;
  top: 2.8rem;
  padding: 0;
  margin: 0;
  height: calc(100vh - 2.8rem - (65px + 2rem));
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
  margin: 1rem 0 0 0;
  padding-left: 1rem;
  color: #633f86;
  border-radius: 0.3rem;
}
.n-filename:hover{
  background-color: #a4a8d455;
}
.n-syntax{
  list-style: none;
  padding: 0;
  margin: 0;
}
.n-syntax li{
  color: #734f96;
  font-size: 0.95rem;
  padding-left: 1.5rem;
  margin-bottom: 0.2rem;
  border-radius: 0.3rem;
  overflow-x: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;
}
.n-syntax li::-webkit-scrollbar {
  display: none;
}
.n-syntax li:hover{
  background-color: #a4a8d455;
}
.dn{
  display: none !important;
}
.bottom{
  position: sticky;
  display: grid;
  place-content: center;
  margin: 0;
  bottom: 0;
  border-top: solid 1px #734f9655;
}
.time{
  width: 128px;
}
main{
  min-width: 0;
}
.m-filename{
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
.m-syntax_name{
  position: relative;
  margin: 0;
  padding-top: 1rem;
  padding-left: 2rem;
  color: #734f96;
}
.m-syntax_name::before{
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
.anchor{
  position: absolute;
  top: 0;
  left: -200px;
}
.doc{
  width: 98%;
  border: solid 2px #cab8d9;
  overflow-x: auto;
  border-radius: 6px;
}
.doc::-webkit-scrollbar {
  height: 8px;
}
.doc::-webkit-scrollbar-thumb {
  background: #7025bb66;
  border-radius: 10px;
}
.doc p{
  line-height: 1.2rem;
  margin-left: 2rem;
  font-size: 1rem;
  font-family: 'Noto Sans JP', sans-serif;
}
.code{
  width: 98%;
}
.code::-webkit-scrollbar {
  height: 10px;
}
.code::-webkit-scrollbar-thumb {
  background: #9933ffbb;
  border-radius: 8px;
}
a{
  color: inherit;
  text-decoration: none;
}
:root {
  --ggy: 1;
  --ggk: 1.5;
}
 .gg-copy {
 box-sizing: border-box;
 position: relative;
 display: inline-block;
 transform: scale(var(--ggy,1));
 width: 14px;
 height: 18px;
 border: 2px solid;
 margin-left: 0.5rem;
 opacity: 0.2;
}
.gg-copy:hover{
  opacity: 1;
}
.gg-copy::after,
.gg-copy::before {
  content: "";
  display: block;
  box-sizing: border-box;
  position: absolute;
}
.gg-copy::before {
  background:
  linear-gradient( to left,
  currentColor 5px, transparent 0)
  no-repeat right top/5px 2px,
  linear-gradient( to left,
  currentColor 5px, transparent 0)
  no-repeat left bottom/ 2px 5px;
  box-shadow: inset -4px -4px 0 -2px;
  bottom: -6px;
  right: -6px;
  width: 14px;
  height: 18px;
}
.gg-copy::after {
  width: 6px;
  height: 2px;
  background: currentColor;
  left: 2px;
  top: 2px;
  box-shadow: 0 4px 0,0 8px 0;
}
input {
 position: absolute;
 opacity: 0;
}
.gg-check {
  box-sizing: border-box;
  position: relative;
  display: inline-block;
  transform: scale(var(--ggk,1));
  width: 18px;
  height: 18px;
  border: 2px solid transparent;
  border-radius: 100px;
  margin-left: 0.5rem;
}
.gg-check::after {
  content: "";
  display: block;
  box-sizing: border-box;
  position: absolute;
  left: 3px;
  top: -1px;
  width: 6px;
  height: 10px;
  border-width: 0 2px 2px 0;
  border-style: solid;
  transform-origin: bottom left;
  transform: rotate(45deg);
}
</style>"#;
