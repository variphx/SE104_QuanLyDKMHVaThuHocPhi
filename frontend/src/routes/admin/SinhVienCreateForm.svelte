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
    let success_count = 0;

    for (let payload of payloads) {
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

      success_count += 1;
    }
    alert(`${success_count} sinh viên tạo thành công`);
  }
</script>

<form on:submit={submitHandler}>
  <label class="lable mb-4">
    <div class="flex flex-col mb-4">
      <span
        class="h2 font-bold mx-auto rounded-full px-8 py-2 variant-glass-surface"
      >
        Sinh viên
      </span>
    </div>
    <input type="file" class="input" bind:files on:change={filesParse} />
  </label>
  <div class="flex flex-col">
    <button type="submit" class="mx-auto btn variant-filled-primary">
      Tạo sinh viên
    </button>
  </div>
</form>
