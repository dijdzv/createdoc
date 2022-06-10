<?php

/**
 * txtファイルを読み込み配列に格納する
 *
 * @param string $filepath
 * @return array
 */
function read_txt_file(string $filepath): array
{
  $fp = fopen($filepath, 'r');
  flock($fp, LOCK_SH);
  $row_list = [];
  while ($row = fgets($fp)) {
    $row = str_replace("\r", '', $row);
    $row = str_replace("\n", '', $row);
    $row_list[] = $row;
  }
  fclose($fp);
  return $row_list;
}

/**
 * txtファイルに
 *
 * @param string $filepath
 * @param array $content
 * @return void
 */
function add_txt_data(string $filepath, array $content)
{
  $fp = fopen($filepath, 'a');
  flock($fp, LOCK_EX);
  fputs($fp, $content . "\r\n");
  fclose($fp);
}


function write_txt_file($filepath, $row_list)
{
  $fp = fopen($filepath, 'w');
  flock($fp, LOCK_EX);
  foreach ($row_list as $row) {
    fputs($fp, $row . "\r\n");
  }
  fclose($fp);
}

/**
 * 実行する度に+1するカウント
 *
 * @param  string $filepath ファイルパス
 * @param  int    $row_num   カウントしたい行番号
 * @return void
 */
function count_num($filepath, $row_num)
{
  $file = file($filepath);
  $count = $file[$row_num];
  $count = str_replace("\r", '', $count);
  $count = str_replace("\n", '', $count);
  $count++;
  $file[$row_num] = $count . "\r\n";
  file_put_contents($filepath, $file);
}

//YYYYmmddHHiiss
// function add_now_time_txt($filepath)
// {
//   date_default_timezone_set('Asia/Tokyo');
//   $dateTime = new DateTime();
//   $YmdHis = $dateTime->format('YmdHis');
//   add_txt_data($filepath, $YmdHis);
// }
