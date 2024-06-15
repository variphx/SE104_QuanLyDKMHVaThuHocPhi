<script lang="ts">
  import type { PageServerData } from "./$types";

  interface SinhVienChuaDongHocPhi {
    id: string;
    ten: string;
    tong: number;
    da_thanh_toan: number;
  }

  export let data: PageServerData;
  const sinh_vien_chua_dong_hoc_phis: Promise<SinhVienChuaDongHocPhi[]> =
    data.sinh_vien_chua_dong_hoc_phis;
</script>

{#await sinh_vien_chua_dong_hoc_phis then sinh_viens}
  {#each sinh_viens as sinh_vien (sinh_vien.id)}
    <div class="w-screen">
      <div class="w-fit mx-auto my-4 py-2 px-4 variant-glass-surface rounded-2xl">
        <div class="h4">
          Sinh viên <b>
            {sinh_vien.ten}
          </b>, MSSV:
          <b>
            {sinh_vien.id}
          </b>
        </div>
        <div class="h4">
          Tổng học phí: <b>
            {sinh_vien.tong}
          </b>
        </div>
        <div class="h4">
          Đã thanh toán: <b>
            {sinh_vien.da_thanh_toan}
          </b>
        </div>
        <div class="h4">
          Còn lại: <b>
            {sinh_vien.tong - sinh_vien.da_thanh_toan}
          </b>
        </div>
      </div>
    </div>
  {/each}
{/await}
