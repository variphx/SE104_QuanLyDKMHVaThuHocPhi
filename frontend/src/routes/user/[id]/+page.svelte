<script lang="ts">
  import type { PageServerData } from "./$types";

  export let data: PageServerData;

  const sinh_vien = data.sinh_vien;
</script>

<div class="grid grid-rows-[auto_1fr] h-screen">
  <div class="w-screen">
    <h1
      class="h2 md:h1 uppercase mx-auto variant-glass-surface px-8 py-4 rounded-full mt-8 w-fit"
    >
      Thông tin sinh viên
    </h1>
  </div>

  <div class="mx-auto my-auto w-2/3 md:w-2/5">
    <div class="grid grid-cols-2 gap-2 bg-surface-50 px-8 py-4 rounded-3xl">
      <div class="variant-glass-surface rounded-2xl flex flex-col">
        <span class="my-auto mx-8">
          <b>Họ và tên</b>
        </span>
      </div>
      <div class="px-8 py-2 rounded-full">
        {sinh_vien.ten}
      </div>

      <div class="variant-glass-surface rounded-2xl flex flex-col">
        <span class="my-auto mx-8">
          <b>Mã số sinh viên</b>
        </span>
      </div>
      <div class="px-8 py-2 rounded-full">
        {sinh_vien.id}
      </div>

      <div class="variant-glass-surface rounded-2xl flex flex-col">
        <span class="my-auto mx-8">
          <b>Quê quán</b>
        </span>
      </div>
      <div class="px-8 py-2 rounded-full">
        {sinh_vien.que_quan.thanh_pho}, {sinh_vien.que_quan.tinh}
      </div>

      <div class="variant-glass-surface rounded-2xl flex flex-col">
        <span class="my-auto mx-8">
          <b>Đối tượng</b>
        </span>
      </div>
      <div class="px-8 py-2 rounded-full">
        {sinh_vien.doi_tuong}
      </div>

      <div class="variant-glass-surface rounded-2xl flex flex-col">
        <span class="my-auto mx-8">
          <b>Ngành</b>
        </span>
      </div>
      <div class="px-8 py-2 rounded-full">
        {sinh_vien.nganh}
      </div>

      <div class="variant-glass-surface rounded-2xl flex flex-col">
        <span class="my-auto mx-8">
          <b>Khoa</b>
        </span>
      </div>
      <div class="px-8 py-2 rounded-full">
        {sinh_vien.khoa}
      </div>

      <div class="variant-glass-surface rounded-2xl flex flex-col">
        <span class="my-auto mx-8">
          <b>Nhập học</b>
        </span>
      </div>
      <div class="px-8 py-2 rounded-full">
        Học kỳ {sinh_vien.hoc_ky_nhap_hoc.ten}, Năm học {sinh_vien
          .hoc_ky_nhap_hoc.nam_hoc}-{sinh_vien.hoc_ky_nhap_hoc.nam_hoc + 1}
      </div>
    </div>
  </div>
</div>
