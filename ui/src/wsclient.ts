import type { Mail } from "./mail_store";
import { mails } from "./mail_store";

const router = new Map<string, (o: object) => void>([
  ["MailList", (o: object) => {
    let new_mail = <Mail[]>o;

    mails.mailList(new_mail);
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

    if (route) route(frame.payload[key]);
  }
}

wsc.onclose = (e: CloseEvent) => {
  console.log("Connection lost:", e.reason);
}
