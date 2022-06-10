<?php

/**
 * 2次元配列から1次元配列を抜き取る
 *
 * @param  array $row_list 抜き取られる2次元配列
 * @param  int $colNum 抜き取りたいカラム番号
 * @return array 抜き取られた1次元配列
 */
function two_to_one(array $row_list, int $colNum): array
{
  $list = [];
  foreach ($row_list as $row) {
    $list[] = $row[$colNum];
  }
  return $list;
}

/**
 * ファイルに追加せずに追加したい値が何番目(key)か返す
 *
 * @param  array $list 1次元配列のみの配列
 * @param  int $add_num 追加したい値
 * @return int 何番目
 */
function what_num(array $list, int $add_num): int
{
  usort($list, function ($point_1, $point_2) {
    return $point_1 <=> $point_2;
  });
  $list[] = $add_num;

  return array_keys($list, $add_num)[count(array_keys($list, $add_num)) - 1];
}
