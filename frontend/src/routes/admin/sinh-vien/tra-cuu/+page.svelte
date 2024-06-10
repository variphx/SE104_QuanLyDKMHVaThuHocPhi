<script lang="ts">
  import SinhVienTable from "./SinhVienTable.svelte";
</script>

<div class="mx-auto w-[90%]">
  <input class="input w-1/2 mb-20 mt-16" type="text" placeholder="Nhập mã số sinh viên cần tìm" />
</div>

<div class="mx-auto w-[90%] justify-center items-center flex">
  <SinhVienTable></SinhVienTable>
</div>
