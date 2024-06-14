<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<div class="h2 m-4 px-8 py-4 variant-filled-primary w-fit rounded-full">
  Học phí sinh viên: <b>{data.id_sinh_vien}</b>
</div>

<div class="flex flex-col p-8">
  {#await data.hoc_phis then hoc_phis}
    {#each hoc_phis as hoc_ky}
      <div
        class="w-fit mx-auto px-16 py-8 variant-glass-surface grid grid-cols-2 gap-x-4 gap-y-2 h4 rounded-3xl"
      >
        <div>
          <b>Năm học:</b>
          {hoc_ky.nam_hoc}
        </div>
        <div>
          <b>Học kỳ:</b>
          {hoc_ky.ten_hoc_ky}
        </div>
        <div>
          <b>Tổng học phí:</b>
          {hoc_ky.tong}
        </div>
        <div>
          <b>Đã thanh toán:</b>
          {hoc_ky.da_thanh_toan}
        </div>
        <div>
          <b>Còn lại:</b>
          {hoc_ky.tong - hoc_ky.da_thanh_toan}
        </div>
      </div>
    {/each}
  {/await}
</div>
