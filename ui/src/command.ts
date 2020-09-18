import { wsc } from "./wsclient";
import { mail_store } from "./mail_store";
import { v4 as uuid } from "uuid";

export function command(cmd_string: string) {
  let cmd_char = cmd_string[0];
  let cmd = cmd_string.substring(1);

  if (cmd_char === '/') {
    let cid = uuid();
    let request = JSON.stringify({
      cid,
      payload: {
        MailSearch: cmd,
      },
    });

    mail_store.expectedCid(cid);
    wsc.send(request);
  }
}
