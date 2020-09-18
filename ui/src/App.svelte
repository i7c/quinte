<script lang="ts">
  import MailList from "./MailList.svelte";
  import { mail_store } from "./mail_store";
  import { command } from "./command";

  let nmode_keybindings: Map<string, () => void> = new Map([
    ["j", () => mail_store.select_next()],
    ["k", () => mail_store.select_prev()],
    [
      "Enter",
      () => {
        let focus = document.activeElement;
        if (focus.id === "omnibar" && focus instanceof HTMLElement) {
          command(omnibar_text);
          focus.blur();
        }
      },
    ],
    [
      "/",
      () => {
        let omnibar = document.getElementById("omnibar");
        omnibar.focus();
        omnibar_text = "";
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

  let omnibar_text: string = "";
</script>

<style>
  input {
    width: 100%;
  }
</style>

<main>
  <input type="text" id="omnibar" bind:value={omnibar_text} />
  <MailList />
</main>
<svelte:window on:keydown={handleKeydown} />
