import { v4 as uuid } from "uuid";
import { writable } from "svelte/store";
import { wsc } from "./wsclient";

export interface Mail {
  content_type: string;
  date: number;
  from: string;
  path: string;
  subject: string;
  to: string;
}

class MailStore {
  readonly expected_cid: string = "";
  readonly mails: Mail[] = [];
  readonly selected_item: number = 0;

  // These are all derived values that cannot be set manually
  readonly page: Mail[] = [];
  readonly page_size: number = 20;
  readonly page_count: number = 1;
  readonly page_active: number = 0;
  readonly page_selected_item: number = 0;

  constructor(vals: Partial<MailStore>) {
    Object.assign(this, vals);
  }

  static create(m: Mail[]): MailStore {
    return new MailStore({}).update_mails("", m);
  }

  update_mails(cid: string, m: Mail[]): MailStore {
    if (cid !== this.expected_cid) return this;
    return this.update({
      mails: m,
      selected_item: 0,
    });
  }

  select_next(): MailStore { return this.update({ selected_item: this.selected_item + 1 }); }
  select_prev(): MailStore { return this.update({ selected_item: this.selected_item - 1 }); }
  set_expected_cid(cid: string): MailStore { return this.update({ expected_cid: cid }); }

  update(vals: Partial<MailStore>): MailStore {
    let nms: MailStore = new MailStore({ ...this, ...vals });
    return nms.normalize();
  }

  normalize(): MailStore {
    // force global selected item into bounds
    let nindex = Math.max(0, Math.min(this.mails.length - 1, this.selected_item));

    let page_count = Math.floor(this.mails.length / this.page_size) + 1;
    let page_active = Math.floor(nindex / this.page_size);
    let page_selected_item = nindex % this.page_size;
    let page = this.mails.slice(page_active * this.page_size, (page_active + 1) * this.page_size);

    return new MailStore({
      ...this,
      selected_item: nindex,
      page_count,
      page_active,
      page_selected_item,
      page,
    });
  }
}

class MailStoreApi {
  store = writable(MailStore.create([]));

  subscribe = this.store.subscribe;
  update = this.store.update;

  select_next() { this.update(s => s.select_next()); }
  select_prev() { this.update(s => s.select_prev()); }

  search(query: string) {
    let cid = uuid();
    let request = JSON.stringify({
      cid,
      payload: {
        MailSearch: query,
      },
    });

    this.update(s => s.set_expected_cid(cid));
    wsc.send(request);
  }

  update_mails(cid: string, m: Mail[]) { this.update(s => s.update_mails(cid, m)); }
}

export const mail_store = new MailStoreApi();
