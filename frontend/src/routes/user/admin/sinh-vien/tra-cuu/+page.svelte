<script lang="ts">
  import type { PageServerData } from "./$types";
  import SinhVienDatatable from "./SinhVienDatatable.svelte";

  export let data: PageServerData;
  const sinh_viens_json = data.sinh_viens;
</script>

{#await sinh_viens_json then sinh_viens}
  <div class="flex flex-col my-16">
    <div class="mx-auto">
      <SinhVienDatatable data={sinh_viens}></SinhVienDatatable>
    </div>
  </div>
{/await}
