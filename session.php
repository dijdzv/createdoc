<?php

/**
 * セッションを全て消す fin
 *
 * @param string ...$session_names 全てのセッションの名前
 * @return void
 */
// function delete_session(string ...$session_names): void
// {
//   foreach ($session_names as $name) {
//     unset($_SESSION[$name]);
//   }
//   $_SESSION = [];
//   session_destroy();
// }


/**
 * セッション変数を配列に格納する
 *
 * @param string ...$session_names 格納したいセッションの名前
 * @return array セッションの値が格納された配列
 */
function get_session(string ...$session_names): array
{
  $data = [];
  foreach ($session_names as $name) {
    $data[] = $_SESSION[$name];
  }
  return $data;
}
