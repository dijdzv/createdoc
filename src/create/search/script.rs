pub const SEARCH_SCRIPT: &str = "<script>
let searchInput = document.getElementById('search-input');
document.addEventListener('keyup', (e) => {
  if (e.key === 's') {
    searchInput.focus();
  }
});
searchInput.addEventListener('input',()=>{
    let v = searchInput.value;
    console.log(v);
})
// searchData宣言済み
console.log(searchData);
</script>";
