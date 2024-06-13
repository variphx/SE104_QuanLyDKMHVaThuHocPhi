<script lang="ts">
  import Papa from "papaparse";

  interface MonHocMoCreatePayload {
    id_hoc_ky: string;
    id_mon_hoc: string;
    id_chuong_trinh_hoc: string;
  }

  let payloads: MonHocMoCreatePayload[];
  let files: FileList;

  async function parseFiles() {
    if (!files) {
      throw new Error("files undefined");
    }

    const file: File = files[0];

    Papa.parse(file, {
      complete: function (result: Papa.ParseResult<MonHocMoCreatePayload>) {
        payloads = result.data;
        console.log(payloads);
      },

      header: true,
      worker: true,
      skipEmptyLines: true,
    });
  }

  async function submitHandler() {
    if (!payloads) {
      throw new Error("payloads undefined");
    }

    for (const payload of payloads) {
      const request = await fetch("http://localhost:8080/api/mon-hoc-mo/post", {
        headers: {
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(payload),
      });

      if (request.ok) {
        console.log("Mở môn học thành công");
      } else {
        console.log("Mở môn học thất bại");
      }
    }
  }
</script>

<form on:submit={submitHandler}>
  <lable class="lable">
    <span> Mở môn học </span>
    <input type="file" class="input" bind:files on:change={parseFiles} />
  </lable>
  <button class="btn variant-filled-primary" type="submit"> Mở </button>
</form>
