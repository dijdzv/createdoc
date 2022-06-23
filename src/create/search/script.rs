pub const SEARCH_SCRIPT: &str = "<script>
let searchInput = document.getElementById('search-input');
document.addEventListener('keyup', (e) => {
  if (e.key === 's') {
    searchInput.focus();
  }
});
searchInput.addEventListener('input',()=>{
    let v = searchInput.value;
    for (let i = 0; i < searchData.length; i++){
     if (searchData[i].indexOf(v) != -1 && v){
        searchList[i].classList.remove('dn');
     }else{
        searchList[i].classList.add('dn');
     }
    }
})
const searchList = document.getElementsByClassName('search-list');
// searchData宣言済み
</script>";
