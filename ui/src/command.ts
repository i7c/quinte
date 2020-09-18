import { mail_store } from "./mail_store";

export function command(cmd_string: string) {
  let cmd_char = cmd_string[0];
  let cmd = cmd_string.substring(1);

  if (cmd_char === '/') {
    mail_store.search(cmd);
  }
}
