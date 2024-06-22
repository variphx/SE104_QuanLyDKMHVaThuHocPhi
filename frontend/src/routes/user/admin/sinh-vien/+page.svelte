<script lang="ts">
  import SinhVienCreateForm from "$lib/components/SinhVienCreateForm.svelte";
  import type { PageServerData } from "./$types";

  export let data: PageServerData;
  const tinh_options = data.tinh_options;
  const doi_tuong_options = data.doi_tuong_options;
  const khoa_options = data.khoa_options;
</script>

<SinhVienCreateForm {tinh_options} {doi_tuong_options} {khoa_options}
></SinhVienCreateForm>
