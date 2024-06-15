<script lang="ts">
  import type { PageData } from "./$types";
  import MonHocMoTable from "./MonHocMoTable.svelte";

  export let data: PageData;
</script>

{#await data.mon_hocs then mon_hocs}
  <div class="w-4/5 mx-auto">
    <MonHocMoTable id_sinh_vien={data.id_sinh_vien} {mon_hocs}></MonHocMoTable>
  </div>
{/await}
