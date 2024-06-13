<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;

  const id_sinh_vien = data.id_sinh_vien;
  let id_hoc_ky: string;

  interface HocPhi {
    tong: number;
    da_thanh_toan: number;
  }

  let hoc_phi: HocPhi;

  const submitHandler = async () => {
    const request = await fetch("http://localhost:8080/api/hoc-phi/get", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id_sinh_vien: id_sinh_vien,
        id_hoc_ky: id_hoc_ky,
      }),
    });
  };
</script>

<select class="select" bind:value={id_hoc_ky}>
  <option value="202201">Năm học 2022-2023, học kỳ Một</option>
</select>
