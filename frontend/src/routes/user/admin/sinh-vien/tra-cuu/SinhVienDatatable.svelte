<script lang="ts">
  import Pagination from "$lib/components/datatable/Pagination.svelte";
  import RowCount from "$lib/components/datatable/RowCount.svelte";
  import RowsPerPage from "$lib/components/datatable/RowsPerPage.svelte";
  import Search from "$lib/components/datatable/Search.svelte";
  import ThFilter from "$lib/components/datatable/ThFilter.svelte";
  import ThSort from "$lib/components/datatable/ThSort.svelte";

  export let data: SinhVien[];

  //Import handler from SSD
  import { DataHandler } from "@vincjo/datatables";
  import type { SinhVien } from "./sinh_vien_interface";

  //Init data handler - CLIENT
  const handler = new DataHandler(data, { rowsPerPage: 5 });
  const rows = handler.getRows();
</script>

<div class=" overflow-x-auto space-y-2">
  <header class="flex justify-between">
    <Search {handler} />
    <RowsPerPage {handler} />
  </header>

  <table class="table table-hover table-compact w-full table-auto">
    <thead>
      <tr>
        <ThSort {handler} orderBy="id">MSSV</ThSort>
        <ThSort {handler} orderBy="ten">Họ và tên</ThSort>
        <ThSort {handler} orderBy="id_gioi_tinh">Giới tính</ThSort>
        <ThSort {handler} orderBy="ngay_sinh">Ngày sinh</ThSort>
        <ThSort {handler} orderBy="id_que_quan">Mã quê quán</ThSort>
        <ThSort {handler} orderBy="id_chuong_trinh_hoc"
          >Mã chương trình học</ThSort
        >
      </tr>
      <tr>
        <ThFilter {handler} filterBy="id" />
        <ThFilter {handler} filterBy="ten" />
        <ThFilter {handler} filterBy="id_gioi_tinh" />
        <ThFilter {handler} filterBy="ngay_sinh" />
        <ThFilter {handler} filterBy="id_que_quan" />
        <ThFilter {handler} filterBy="id_chuong_trinh_hoc" />
      </tr>
    </thead>
    <tbody>
      {#each $rows as row}
        <tr>
          <td>{row.id}</td>
          <td>{row.ten}</td>
          <td>{row.id_gioi_tinh}</td>
          <td>{row.ngay_sinh}</td>
          <td>{row.id_que_quan}</td>
          <td>{row.id_chuong_trinh_hoc}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  <footer class="flex justify-between">
    <RowCount {handler} />
    <Pagination {handler} />
  </footer>
</div>
