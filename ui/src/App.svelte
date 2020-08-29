<script lang="ts">
  import MailList from "./MailList.svelte";
  import { mails } from "./mail_store";
  import { wsc } from "./wsclient";

  let nmode_keybindings: Map<string, () => void> = new Map([
    ["j", () => mails.selectDown()],
    ["k", () => mails.selectUp()],
    [
      "Enter",
      () => {
        let focus = document.activeElement;
        if (focus.id === "omnibar" && focus instanceof HTMLElement) {
          let request = JSON.stringify({
            cid: "search/1",
            payload: {
              MailSearch: command.substring(1),
            },
          });

          wsc.send(request);
          focus.blur();
        }
      },
    ],
    [
      "/",
      () => {
        let omnibar = document.getElementById("omnibar");
        omnibar.focus();
        command = "";
      },
    ],
  ]);

  function handleKeydown(e: KeyboardEvent) {
    let focus = document.activeElement;

    if (e.key === "Escape" && focus instanceof HTMLElement) {
      focus.blur();
    } else if (focus.nodeName.toLowerCase() === "input" && e.key !== "Enter")
      return;

    let handler = nmode_keybindings.get(e.key);
    if (handler) handler();
  }

  let command: string = "";
</script>

<style>
  input {
    width: 100%;
  }
</style>

<main>
  <input type="text" id="omnibar" bind:value={command} />
  <MailList />
</main>
<svelte:window on:keydown={handleKeydown} />
