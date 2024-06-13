<script>
  import ChuongTrinhHocCreateForm from "./ChuongTrinhHocCreateForm.svelte";
  import MonHocCreateForm from "./MonHocCreateForm.svelte";
  import MonHocMoCreateForm from "./MonHocMoCreateForm.svelte";
  import SinhVienCreateForm from "./SinhVienCreateForm.svelte";
</script>

<div class="p-20">
  <div class="min-h-full grid grid-cols-2">
    <div class="m-4 p-4 rounded-2xl variant-glass-surface">
      <SinhVienCreateForm></SinhVienCreateForm>
    </div>
    <div class="m-4 p-4 rounded-2xl variant-glass-surface">
      <ChuongTrinhHocCreateForm></ChuongTrinhHocCreateForm>
    </div>
    <div class="m-4 p-4 rounded-2xl variant-glass-surface">
      <MonHocCreateForm></MonHocCreateForm>
    </div>
    <div class="m-4 p-4 rounded-2xl variant-glass-surface">
      <MonHocMoCreateForm></MonHocMoCreateForm>
    </div>
  </div>
</div>
