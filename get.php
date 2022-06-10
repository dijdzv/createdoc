<?php
$dir_path = dirname(__FILE__);
require_once "{$dir_path}/file.php";

/**
 * 指定した複数のデータが同レコードに存在しているidを得る
 *
 * @param string $filepath ファイルパス
 * @param array ...$values 指定するデータ [値,col番号]...
 * @return string|boolean idかfalse
 */
function get_id_multiple_exist_csv_record(string $filepath, array ...$values): string|bool
{
  $row_list = read_csv_file($filepath);
  $count = count($values);
  foreach ($row_list as $row) {
    foreach ($values as $key => $value) {
      if ($row[$value[1]] === $value[0]) {
        if ($count === $key + 1) {
          return $row[0];
        }
      } else {
        break;
      }
    }
  }
  return false;
}

/**
 * 一意の値からデータを得る
 *
 * @param string $filepath ファイルパス
 * @param string $id 指定するid
 * @param integer $id_col_num idのcol番号
 * @param integer $col_num 得たいデータのcol番号
 * @return string|boolean データかfalse
 */
function get_data_from_id_csv_file(string $filepath, string $id, int $id_col_num, int $col_num): string|bool
{
  $row_list = read_csv_file($filepath);
  foreach ($row_list as $row) {
    if ($row[$id_col_num] === $id) {
      return $row[$col_num];
    }
  }
  return false;
}
