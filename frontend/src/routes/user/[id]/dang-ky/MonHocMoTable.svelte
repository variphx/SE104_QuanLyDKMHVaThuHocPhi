<script lang="ts">
  interface MonHocMo {
    id_mon_hoc: string;
    ten: string;
    loai: string;
    so_tin_chi: number;
  }

  export let mon_hocs: MonHocMo[];
  let display_mon_hocs: MonHocMo[] = mon_hocs;
  export let filter_string: string;
  let filter_strings: string[];

  function filterMonHocs(filter_string: string) {
    const filter_string_trim = filter_string.trim();

    if (filter_string_trim == "") {
      display_mon_hocs = mon_hocs;
      return;
    }

    filter_strings = filter_string_trim
      .split(new RegExp(/\s+|,/))
      .filter((str) => str != "");
    display_mon_hocs = mon_hocs.filter((mon_hoc) =>
      filter_strings.includes(mon_hoc.id_mon_hoc),
    );
  }

  $: filterMonHocs(filter_string);

  async function submitHandler() {}
</script>

<div class="table-container">
  <table class="table table-hover">
    <thead>
      <tr>
        <th>Mã môn học</th>
        <th>Tên môn học</th>
        <th>Loại môn học</th>
        <th>Số tín chỉ</th>
      </tr>
    </thead>
    <tbody>
      {#each display_mon_hocs as mon_hoc}
        <tr>
          <td>{mon_hoc.id_mon_hoc}</td>
          <td>{mon_hoc.ten}</td>
          <td>{mon_hoc.loai}</td>
          <td>{mon_hoc.so_tin_chi}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<button
  type="button"
  class="btn variant-filled-primary"
  on:click={submitHandler}>Đăng ký</button
>
