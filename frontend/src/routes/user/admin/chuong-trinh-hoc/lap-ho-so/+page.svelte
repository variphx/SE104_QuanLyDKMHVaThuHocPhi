<script lang="ts">
  import ChuongTrinhHocCreateForm from "$lib/components/ChuongTrinhHocCreateForm.svelte";
  import type { PageServerData } from "./$types";

  export let data: PageServerData;
  const nganhs = data.nganhs;
</script>

  <ChuongTrinhHocCreateForm {nganhs}></ChuongTrinhHocCreateForm>
