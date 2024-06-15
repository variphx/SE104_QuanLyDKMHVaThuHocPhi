<script lang="ts">
  import { InputChip } from "@skeletonlabs/skeleton";

  interface MonHocMo {
    id_mon_hoc: string;
    ten: string;
    loai: string;
    so_tin_chi: number;
  }

  export let mon_hocs: MonHocMo[];
  export let id_sinh_vien: string;

  async function submitHandler() {
    if (!selected_id_mon_hocs) {
      alert("Chưa chọn môn học");
      return;
    }

    let mon_hocs_created_count = 0;

    for (let id_mon_hoc of selected_id_mon_hocs) {
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

  let selected_id_mon_hocs: string[];

  const validateIdMonHoc = (id: string) => {
    for (let mon_hoc of mon_hocs) {
      if (mon_hoc.id_mon_hoc == id) {
        return true;
      }
    }

    return false;
  };
</script>

<div class="mx-auto mb-8 mt-4">
  <InputChip
    name="selected_id_mon_hocs"
    bind:value={selected_id_mon_hocs}
    placeholder="Nhập mã môn cần chọn"
    allowUpperCase={true}
    required={true}
    validation={validateIdMonHoc}
  ></InputChip>
</div>

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
      {#each mon_hocs as mon_hoc}
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
