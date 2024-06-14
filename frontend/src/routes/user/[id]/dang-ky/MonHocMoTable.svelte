<script lang="ts">
  interface MonHocMo {
    id_mon_hoc: string;
    ten: string;
    loai: string;
    so_tin_chi: number;
  }

  export let mon_hocs: MonHocMo[];
  export let id_sinh_vien: string;
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

  async function submitHandler() {
    if (!filter_strings) {
      alert("Chưa chọn môn học");
      return;
    }

    let mon_hocs_created_count = 0;

    for (let id_mon_hoc of filter_strings) {
      const request = await fetch("http://localhost:8080/api/dkmh/post", {
        headers: {
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify({
          id_mon_hoc: id_mon_hoc,
          id_sinh_vien: id_sinh_vien,
        }),
      });

      if (request.ok) {
        mon_hocs_created_count += 1;
      }

      alert(`${mon_hocs_created_count} môn học đăng ký thành công`);
    }
  }
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

<div class="min-w-full mt-12">
  <div class="w-1/4 mx-auto">
    <button
      type="button"
      class="btn variant-filled-primary w-full items-center justify-center"
      on:click={submitHandler}>Đăng ký</button
    >
  </div>
</div>
