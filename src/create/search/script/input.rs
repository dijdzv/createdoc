pub const INPUT_SCRIPT: &str = "<script>
document.addEventListener('keyup', (e) => {
  if (e.key === 's') {
    document.getElementById('search-input').focus();
  }
});
</script>";
