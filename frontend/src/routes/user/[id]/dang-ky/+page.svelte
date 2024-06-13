<script lang="ts">
  import type { PageData } from "./$types";
  import MonHocMoTable from "./MonHocMoTable.svelte";

  export let filter_string: string = "";

  export let data: PageData;
</script>

<input class="input" bind:value={filter_string} />

{#await data.mon_hocs then mon_hocs}
  <MonHocMoTable {mon_hocs}{filter_string}></MonHocMoTable>
{/await}
