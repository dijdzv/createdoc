<?php

/**
 * 昇順・降順
 *
 * @param array $array_2D ソートしたい2次元配列
 * @param int $col_num 比較する配列番号
 * @param string $order ASCかDESC
 * @return array ソート後の2次元配列
 */
function order(array $array_2D, int $col_num, string $order): array
{
  for ($j = 0; $j < count($array_2D) - 1; $j++) {
    for ($i = 0; $i < count($array_2D) - 1 - $j; $i++) {
      if ($order === 'ASC') {
        if ($array_2D[$i][$col_num] > $array_2D[$i + 1][$col_num]) {
          $buf = $array_2D[$i];
          $array_2D[$i] = $array_2D[$i + 1];
          $array_2D[$i + 1] = $buf;
        }
      } else if ($order === 'DESC') {
        if ($array_2D[$i][$col_num] < $array_2D[$i + 1][$col_num]) {
          $buf = $array_2D[$i];
          $array_2D[$i] = $array_2D[$i + 1];
          $array_2D[$i + 1] = $buf;
        }
      }
    }
  }
  return $array_2D;
}
