<script lang="ts">
  import SinhVienCreateForm from "$lib/components/SinhVienCreateForm.svelte";
  import type { PageServerData } from "./$types";

  export let data: PageServerData;
  const tinh_options = data.tinh_options;
  const doi_tuong_options = data.doi_tuong_options;
  const khoa_options = data.khoa_options;
</script>

<div class="flex flex-col h-screen">
  <div class="mx-auto my-auto p-8 rounded-3xl shadow-2xl">
    <SinhVienCreateForm {tinh_options} {doi_tuong_options} {khoa_options}
    ></SinhVienCreateForm>
  </div>
</div>
