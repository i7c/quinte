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
      <td>→</td>
      <td class="mail-subject">{mail.subject}</td>
      <td>{mail.from}</td>
      <td class="mail-date">{(new Date(mail.date * 1000)).toLocaleString()}</td>
    </tr>
  {/each}
</table>

<style>
  table {
    color: #F8F8F2;
    margin: auto;
  }

  td {
    padding: 4px 10px;
  }

  .mail-subject {
    max-width: 50%;
  }

  .mail-date {
    color: #A6E22E;
  }

  .selected {
    background-color: #75715E;
  }
</style>
