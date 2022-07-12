pub const SCRIPT: &str = r#"<script>
// /* navの表示・非表示 */
// let nFile = document.getElementsByClassName("n-filename");

// for (let f of nFile) {
//   f.addEventListener("click", () => {
//     let id = f.getAttribute("id");
//     let cls = document.getElementsByClassName(id)[0];
//     cls.classList.toggle("dn");
//   });
// }

/* copy */
let copyIcon = document.getElementsByClassName("gg-copy");
for (let c of copyIcon) {
  c.addEventListener("click", () => {
    let copyTarget = c.previousElementSibling;
    copyTarget.select();
    document.execCommand("Copy");
    window.getSelection().removeAllRanges();
    let check = c.nextElementSibling;
    c.classList.toggle("dn");
    check.classList.toggle("dn");
    setTimeout(() => {
      c.classList.toggle("dn");
      check.classList.toggle("dn");
    }, 2000);
  });
}

let pair = document.getElementsByClassName("pair");
let lastPair = pair[pair.length - 1];
lastPair.style.marginBottom = `calc(100vh - ${lastPair.clientHeight}px)`;
</script>"#;
