import type { Mail } from "./mail_store";
import { mail_store } from "./mail_store";

const router = new Map<string, (cid: string, o: object) => void>([
  ["MailList", (cid: string, o: object) => {
    let new_mail = <Mail[]>o;

    mail_store.update_mails(cid, new_mail);
  }],
]);

export const wsc = new WebSocket("ws://127.0.0.1:42337");

wsc.onopen = () => {
  console.log("Connection to server established.");
}

wsc.onmessage = (e: MessageEvent) => {
  let frame = JSON.parse(e.data);

  for (let key in frame.payload) {
    let route = router.get(key);

    if (route) route(frame.cid, frame.payload[key]);
  }
}

wsc.onclose = (e: CloseEvent) => {
  console.log("Connection lost:", e.reason);
}
