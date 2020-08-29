<script lang="ts">
  import { mailstore } from "./mail_store";
</script>

<table>
  <tr>
    <td/>
    <td>Subject</td>
    <td>From</td>
    <td>Date</td>
  </tr>
  {#each $mailstore.mails as mail, i}
    <tr class="{i === $mailstore.selected ? "selected" : "unselected"}">
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
