<?php
$dir_path = dirname(__FILE__);
require_once "{$dir_path}/file.php";

/**
 * 存在チェック csv
 *
 * @param string $filepath ファイルパス
 * @param integer $col_num チェックしたいcol番号
 * @param string $login_id チェックする値
 * @return boolean 存在していればtrue。存在していなければfalse
 */
function check_exist_csv(string $filepath, int $col_num, string $value): bool
{
  $row_list = read_csv_file($filepath);
  foreach ($row_list as $row) {
    if ($row[$col_num] === $value) {
      return true;
    }
  }
  return false;
}

/**
 * 同じレコードに指定した複数個の値がすべて存在しているかどうか
 *
 * @param array ...$values チェックしたい複数個の値とcol番号の配列 [値,col番号]....
 * @return boolean すべて存在していればtrue。すべて存在していなければfalse
 */
function check_multiple_exist_csv_record(string $filepath, array ...$values): bool
{
  $row_list = read_csv_file($filepath);
  $count = count($values);
  foreach ($row_list as $row) {
    foreach ($values as $key => $value) {
      if ($row[$value[1]] === $value[0]) {
        if ($count === $key + 1) {
          return true;
        }
      } else {
        break;
      }
    }
  }
  return false;
}
