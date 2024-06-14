<script lang="ts">
  import type { PageData } from "./$types";
  import MonHocMoTable from "./MonHocMoTable.svelte";

  let filter_string: string = "";
  export let data: PageData;
</script>

<div class="w-4/5 mx-auto mb-8 mt-4">
  <div class="">
    <input class="input" type="search" bind:value={filter_string} />
  </div>
</div>

{#await data.mon_hocs then mon_hocs}
  <div class="w-4/5 mx-auto">
    <MonHocMoTable id_sinh_vien={data.id_sinh_vien} {mon_hocs} {filter_string}
    ></MonHocMoTable>
  </div>
{/await}
