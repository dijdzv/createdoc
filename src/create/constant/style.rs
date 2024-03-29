pub const STYLE: &str = r#"<style>
:root {
  --ggs: 1;
  --ggy: 1;
  --ggk: 1.5;
  --color-standard: #dddddd;
  --color-name: #fdbf35;
  --color-syntax: #2dbfb8;
  /* --color-syntax: #d7acf2; */
  --color-background: #3d3d3d;
  --color-nav-background: #505050;
  --color-nav-border: #717171;
  --color-scrollbar: #717171;
  --color-hover: #00000022;
  /* --color-tag: #2cc92c; */
  --color-tag: #f07df8;
  --color-type: #f4c61f;
}

html {
  scroll-behavior: smooth;
}

* {
  box-sizing: border-box;
}

body {
  background-color: var(--color-background);
  overflow-x: hidden;
  margin: 0;
  height: 100%;
  color: var(--color-standard);
}

.wrap {
  display: flex;
}

nav {
  position: sticky;
  top: 0;
  left: 0;
  width: 250px;
  min-width: 200px;
  height: 100vh;
  background-color: var(--color-nav-background);
}

.html-filename {
  position: sticky;
  top: 0;
  margin: 0;
  padding: 0;
  height: 4rem;
  line-height: 4rem;
  text-align: center;
  border-bottom: solid 2px var(--color-nav-border);
  overflow: hidden;
}

.n-folder {
  position: sticky;
  top: 2.8rem;
  padding: 0;
  margin: 0;
  height: calc(100vh - 4rem - 81px);
  overflow-y: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.n-folder::-webkit-scrollbar {
  display: none;
}

.n-file{
  margin-bottom: 2rem;
}

.n-filename {
  cursor: pointer;
  margin: 0.8rem 0 0 1.8rem;
  padding: 0.2rem 0 0.2rem 0.2rem;
}
.n-filename:hover{
  background-color: var(--color-hover);
}
.n-syntax{
  margin: 0 0 0 1.8rem;
  padding: 0.2rem 0 0.2rem 0.2rem;
  color: var(--color-syntax);
}
/* .n-syntax:hover{
  background-color: var(--color-hover);
} */
.n-target {
  list-style: none;
  padding: 0;
  margin: 0;
}

.n-target li {
  color: var(--color-name);
  font-size: 14px;
  letter-spacing: 0px;
  font-weight: 600;
  font-family: "Fira Sans",Arial,NanumBarunGothic,sans-serif;
  margin-left: 1.8rem;
  padding: 0.2rem 0 0.2rem 0.2rem;
  overflow-x: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;
}
.n-target li:hover{
  background-color: var(--color-hover);
}

.n-target li::-webkit-scrollbar {
  display: none;
}

.dn {
  display: none !important;
}

.bottom {
  position: sticky;
  display: grid;
  place-content: center;
  margin: 0;
  bottom: 0;
  border-top: solid 2px var(--color-nav-border);
}

.time {
  white-space: pre;
}

main {
  min-width: 0;
}

.m-filename {
  margin: 1rem 0 0 1rem;
  padding-left: 0.5rem;
  border-bottom: solid 3px var(--color-standard);
}

.m-syntax{
  color: var(--color-syntax);
  margin: 0.5rem 0 0 1.8rem;
  font-size: 1.2rem;
}

.pair {
  margin-left: 1rem;
  padding: 1rem;
  border-radius: 1rem;
}

.m-target_name {
  position: relative;
  margin: 0;
  padding-top: 1rem;
  padding-left: 2rem;
  color: var(--color-name);
}

.m-target_name::before {
  content: "";
  position: absolute;
  top: 1rem;
  bottom: 0;
  left: 0;
  right: 0;
  border-top: solid 0px var(--color-name);
  border-bottom: solid 2px var(--color-name);
  border-left: solid 20px var(--color-name);
  border-right: solid 0px var(--color-name);
  border-radius: 0px;
  z-index: -1;
}

.doc {
  width: 98%;
  overflow-x: auto;
  border-radius: 4px;
  cursor: default;
}

.doc::-webkit-scrollbar {
  height: 8px;
}

.doc::-webkit-scrollbar-thumb {
  background: var(--color-scrollbar);
}

.doc-p {
  font-family: "Meiryo UI", sans-serif;
  margin-left: 2rem;
  font-size: 1rem;
  line-height: 1.2rem;
  letter-spacing: 0.09rem;
}

.code {
  width: 98%;
}

.code::-webkit-scrollbar {
  height: 10px;
}

.code::-webkit-scrollbar-thumb {
  background: var(--color-scrollbar);
}

a {
  color: inherit;
  text-decoration: none;
}

.gg-copy {
  box-sizing: border-box;
  position: relative;
  display: inline-block;
  transform: scale(var(--ggy, 1));
  width: 14px;
  height: 18px;
  border: 2px solid;
  margin-left: 0.5rem;
  opacity: 0.2;
  cursor: pointer;
}

.gg-copy:hover {
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
    linear-gradient(to left,
      currentColor 5px, transparent 0) no-repeat right top/5px 2px,
    linear-gradient(to left,
      currentColor 5px, transparent 0) no-repeat left bottom/ 2px 5px;
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
  box-shadow: 0 4px 0, 0 8px 0;
}

.hidden-input {
  position: absolute;
  opacity: 0;
  cursor: default;
}

.gg-check {
  box-sizing: border-box;
  position: relative;
  display: inline-block;
  transform: scale(var(--ggk, 1));
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

.tag {
  color: var(--color-tag);
}

.type {
  color: var(--color-type);
}

.search-area{
  width: 100%;
  padding: 1rem;
}

#search{
  display: flex;
}

#search-input{
  margin: 0 0 0 10px;
  width: 100%;
  -webkit-appearance: none;
  -moz-appearance: none;
  outline: none;
  border: 1px solid #f0f0f0;
  border-radius: 2px;
  padding: 6px;
  font-size: 1rem;
  background-color: #f0f0f0;
}

.search-result>ul{
  list-style: none;
  padding: 0;
  margin: 0;
}

.search-list{
  border-bottom: solid 1px #aaa9;
  margin-left: 10px;
  padding: 5px 0 5px 0.2rem;
  font-family: "Fira Sans",Arial,NanumBarunGothic,sans-serif;
}
.search-list:hover{
  background-color: #ffffff33;
}

.s-target_name{
  color: var(--color-name);
  margin-left: 0.5rem;
}

.s-syntax{
  color: var(--color-syntax);
  margin-left: 0.5rem;
}

.gg-search {
    box-sizing: border-box;
    position: relative;
    display: block;
    transform: scale(var(--ggs,1));
    width: 16px;
    height: 16px;
    border: 2px solid;
    border-radius: 100%;
    margin-left: -4px;
    margin-top: -4px
}
.gg-search::after {
    content: "";
    display: block;
    box-sizing: border-box;
    position: absolute;
    border-radius: 3px;
    width: 2px;
    height: 8px;
    background: currentColor;
    transform: rotate(-45deg);
    top: 10px;
    left: 12px
}

</style>"#;
