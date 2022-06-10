<?php

/**
 * csvファイルにレコードを追加する
 * 自動でidを付与する
 *
 * @param string $filepath ファイルパス
 * @param array $contents  追加する配列(idを除く)
 * @return void
 */
function add_csv_record(string $filepath, array $contents)
{
  $new_id = create_id_csv_record($filepath);
  $fp = fopen($filepath, 'a');
  flock($fp, LOCK_EX);
  fputs($fp, implode(',', [$new_id, ...$contents]) . "\r\n");
  fclose($fp);
}

/**
 * csvファイルを読み込み配列に格納する
 *
 * @param string $filepath ファイルパス
 * @return array             配列化したcsvファイル
 */
function read_csv_file(string $filepath): array
{
  $fp = fopen($filepath, 'r');
  flock($fp, LOCK_SH);
  $row_list = [];
  while ($row = fgets($fp)) {
    $row = str_replace("\r", '', $row);
    $row = str_replace("\n", '', $row);
    $row_list[] = explode(',', $row);
  }
  fclose($fp);
  return $row_list;
}

/**
 * csvファイルを上書きする
 *
 * @param string $filepath ファイルパス
 * @param array $row_list  上書きする配列
 * @return void
 */
function write_csv_file(string $filepath, array $row_list)
{
  $fp = fopen($filepath, 'w');
  flock($fp, LOCK_EX);
  foreach ($row_list as $row) {
    fputs($fp, implode(',', $row) . "\r\n");
  }
  fclose($fp);
}

/**
 * 指定したidのレコードを削除する
 *
 * @param string $filepath ファイルパス
 * @param int $id 指定するid
 * @return void
 */
function del_csv_record(string $filepath, int $id)
{
  $row_list = read_csv_file($filepath);
  foreach ($row_list as $key => $row) {
    if ((int)$row[0] === $id) {
      unset($row_list[$key]);
      write_csv_file($filepath, $row_list);
      return;
    }
  }
}

/**
 * 指定したidとカラム番号のデータを書き換えてidを更新する
 *
 * @param  string $filepath ファイルパス
 * @param  int $id 指定するid
 * @param  int $colNum 指定するカラム番号
 * @param  int|string $content 編集に使用する値
 * @return void
 */
function edit_csv_data(string $filepath, int $id, int $col_num, string $content)
{
  $row_list = read_csv_file($filepath);
  $new_id = create_id_csv_record($filepath);
  foreach ($row_list as $key => $row) {
    if ((int)$row[0] === $id) {
      $row_list[$key][$col_num] = $content;
      $row_list[$key][0] = $new_id;
      write_csv_file($filepath, $row_list);
      return;
    }
  }
}

/**
 * 指定したidのレコードを書き換えてidを更新する
 *
 * @param string $filepath ファイルパス
 * @param int $id id
 * @param array $contents 編集に使用する配列
 * @return void
 */
function edit_csv_record(string $filepath, int $id, array $contents)
{
  $row_list = read_csv_file($filepath);
  $new_id = create_id_csv_record($filepath);
  foreach ($row_list as $key => $row) {
    if ((int)$row[0] === $id) {
      $row_list[$key] = [$new_id, ...$contents];
      write_csv_file($filepath, $row_list);
      return;
    }
  }
}

/**
 * レコードに付加するidを生成する
 *
 * @param string $filepath ファイルパス
 * @return int 付加するid
 */
function create_id_csv_record(string $filepath): int
{
  $row_list = read_csv_file($filepath);
  $max = 0;
  foreach ($row_list as $row) {
    if ($max < (int)$row[0]) {
      $max = $row[0];
    }
  }
  $id = $max + 1;
  return $id;
}


/**
 * 指定したidのレコードを取得する
 *
 * @param string $filepath ファイルパス
 * @param int $id id
 * @return array|false レコードかfalseを返す
 */
function get_csv_record(string $filepath, int $id): array|false
{
  $row_list = read_csv_file($filepath);
  foreach ($row_list as $row) {
    if ((int)$row[0] === $id) {
      return $row;
    }
  }
  return false;
}
