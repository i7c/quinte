import { writable } from "svelte/store";

export interface Mail {
  content_type: string;
  date: number;
  from: string;
  path: string;
  subject: string;
  to: string;
}

class MailStore {
  store = writable({
    expectedCid: "",
    mails: [],
    selected: 0,
  });

  subscribe = this.store.subscribe;
  update = this.store.update;

  selectDown() {
    this.update(s => ({ ...s, selected: Math.min(s.selected + 1, s.mails.length - 1) }))
  }

  selectUp() {
    this.update(s => ({ ...s, selected: Math.max(s.selected - 1, 0) }))
  }

  mailList(cid: string, m: Mail[]) {
    this.update(s => {
      if (s.expectedCid !== cid) return s;
      return { ...s, mails: m, };
    });
  }

  expectedCid(cid: string) {
    this.update(s => ({ ...s, expectedCid: cid, }));
  }
}

export const mail_store = new MailStore();
