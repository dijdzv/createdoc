// let file = document.getElementById("csv");
// let func = document.getElementsByClassName("csv")[0];
// csv.onclick = () => {
//   func.classList.toggle("dn");
// };

let nFile = document.getElementsByClassName("n-file");

for (let f of nFile) {
  f.addEventListener("click", () => {
    console.log(f);
    let id = f.getAttribute("id");
    console.log(id);
    let cls = document.getElementsByClassName(id)[0];
    console.log(cls);
    cls.classList.toggle("dn");
  });
}
