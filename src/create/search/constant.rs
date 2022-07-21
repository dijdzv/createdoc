pub const SEARCH_SCRIPT: &str = "<script>
let searchInput = document.getElementById('search-input');
document.addEventListener('keyup', (e) => {
  if (e.key === 's') {
    searchInput.focus();
  }
});
searchInput.addEventListener('input',()=>{
    let sv = searchInput.value;
    sv = sv.split(' ');
    if (sv[0]){
      searchInput.style.marginBottom = '1rem';
    }else{
      searchInput.style.marginBottom = '0';
    }
    for (let i = 0; i < searchData.length; i++){
      if (sv.every(v=>searchData[i].indexOf(v) != -1) && sv[0]){
        searchList[i].classList.remove('dn');
     }else{
        searchList[i].classList.add('dn');
     }
    }
})
const searchList = document.getElementsByClassName('search-list');
// searchData宣言済み
</script>";

pub const SEARCH_INPUT: &str = r#"<form id="search" role="search" onsubmit="return false;"><input type="search" id="search-input" name="search" spellcheck="false" autocomplete="off"
      placeholder="Click or press `S` to search"></form>"#;
