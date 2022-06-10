<?php

/**
 * 8桁の数字を年月日に変換したものを返す
 *
 * @param  int    $eight_digit 8桁の数字
 * @return string              年月日
 */
function date_from_8digit_number(int|string $eight_digit): string
{
  $date = substr_replace($eight_digit, '日', '8', 0);
  $date = substr_replace($date, '月', '6', 0);
  $date = substr_replace($date, '年', '4', 0);

  return $date;
}

/**
 * 改行あり出力
 *
 * @param string $string 表示したい文字列
 * @return void
 */
function println(string $string)
{
  echo $string . '<br>';
}

/**
 * 配列を並べて出力(改行あり)
 *
 * @param string $separator 区切り
 * @param array $array 出力したい配列
 * @return void
 */
function print_ary(string $separator, array $array)
{
  $string = implode($separator, $array);
  echo $string . '<br>';
}

/**
 * 文字化けさせる
 *
 * @param string $string 文字化けさせたい文字列
 * @return string 文字化けした文字列
 */
function garbling(string $string): string
{
  $salt = -9000;

  $array = mb_str_split($string);

  $new_array = [];
  foreach ($array as $key => $value) {
    $new_array[$key] = mb_chr(mb_ord($value) + $salt);
  }

  $garbling = implode($new_array);

  return $garbling;
}
