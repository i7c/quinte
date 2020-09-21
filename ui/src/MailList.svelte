<script lang="ts">
  import { mail_store } from "./mail_store";
</script>

<table>
  <tr>
    <td/>
    <td>Subject</td>
    <td>From</td>
    <td>Date</td>
  </tr>
  {#each $mail_store.page as mail, i}
    <tr class="{i === $mail_store.page_selected_item ? "selected" : "unselected"}">
      <td class="mail-flags">→</td>
      <td class="mail-subject">{mail.subject}</td>
      <td class="mail-from">{mail.from}</td>
      <td class="mail-date">{(new Date(mail.date * 1000)).toLocaleString()}</td>
    </tr>
  {/each}
</table>

<style>
  table {
    color: #F8F8F2;
    margin: auto;
    width: 100%;
  }

  td {
    padding: 4px 10px;
  }

  tr {
    max-height: 10px;
  }

  .mail-subject {
    width: 45%;
  }

  .mail-from {
    width: 30%;
  }

  .mail-date {
    width: 15%;
    color: #A6E22E;
  }

  .selected {
    background-color: #75715E;
  }
</style>
