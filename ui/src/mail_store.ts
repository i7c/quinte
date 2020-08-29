import { subscribe } from "svelte/internal";
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
  mails: Mail[] = [];
  selected: number = 0;

  constructor(init?: Partial<MailStore>) { Object.assign(this, init) }
}

class MailStoreApi {
  static store = writable(new MailStore());

  subscribe = MailStoreApi.store.subscribe;

  selectDown() {
    MailStoreApi.store.update(s => new MailStore({
      ...s,
      selected: Math.min(s.selected + 1, s.mails.length - 1)
    }))
  }

  selectUp() {
    MailStoreApi.store.update(s => new MailStore({
      ...s,
      selected: Math.max(s.selected - 1, 0)
    }))
  }

  mailList(m: Mail[]) {
    MailStoreApi.store.update(s => new MailStore({
      mails: m,
      selected: 0,
    }));
  }
}

export const mails = new MailStoreApi();
