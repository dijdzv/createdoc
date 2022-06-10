<?php

// function mysql_escape($mysql, ...$strings)
// {
//   $escaped_strings = [];
//   foreach ($strings as $str) {
//     $escaped_strings[] = mysqli_escape_string($mysql, $str);
//   }
//   return $escaped_strings;
// }


function read_mysql(object $result): array
{
  $row_list = [];
  while ($row = mysqli_fetch_assoc($result)) {
    $row_list[] = $row;
  }
  return $row_list;
}
