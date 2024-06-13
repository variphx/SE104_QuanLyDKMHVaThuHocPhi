<script lang="ts">
  import Papa from "papaparse";

  let files: FileList;
  let payloads: SinhVienCreatePayload[];

  async function filesParse() {
    const file: File = files[0];
    Papa.parse(file, {
      complete: function (result: Papa.ParseResult<SinhVienCreatePayload>) {
        payloads = result.data;
        console.log(payloads);
      },

      header: true,
      worker: true,
      skipEmptyLines: true,
    });
  }

  interface SinhVienCreatePayload {
    id: string;
    ten: string;
    ngay_sinh: string;
    id_gioi_tinh: string;
    id_que_quan: string;
    id_doi_tuong: string;
    id_chuong_trinh_hoc: string;
  }

  async function submitHandler() {
    for (let payload of payloads) {
      console.log(JSON.stringify(payload));
      const request = await fetch("http://localhost:8080/api/sinh-vien/post", {
        headers: {
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(payload),
      });

      if (!request.ok) {
        continue;
      }

      alert("Sinh viên tạo thành công");
    }
  }
</script>

<form on:submit={submitHandler}>
  <label class="lable">
    <span> Sinh viên </span>
    <input type="file" class="input" bind:files on:change={filesParse} />
  </label>
  <button type="submit" class="btn variant-filled-primary">
    Tạo sinh viên
  </button>
</form>
