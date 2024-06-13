<script lang="ts">
  import Papa from "papaparse";

  let files: FileList;

  interface ChuongTrinhHocCreatePayload {
    id_nganh: string;
    id_hoc_ky: string;
  }

  let payloads: ChuongTrinhHocCreatePayload[] = [];

  async function parseFiles() {
    const file = files[0];
    Papa.parse(file, {
      complete: function (
        result: Papa.ParseResult<ChuongTrinhHocCreatePayload>,
      ) {
        payloads = result.data;
        console.log(payloads);
      },

      header: true,
      worker: true,
      skipEmptyLines: true,
    });
  }

  async function submitHandler() {
    for (let payload of payloads) {
      const request = await fetch(
        "http://localhost:8080/api/chuong-trinh-hoc/post",
        {
          headers: {
            "Content-Type": "application/json",
          },
          method: "POST",
          body: JSON.stringify(payload),
        },
      );

      if (request.ok) {
        alert("Tạo chương trình học thành công");
      }
    }
  }
</script>

<form on:submit={submitHandler}>
  <lable class="lable">
    <span> Chương trình học </span>
    <input class="input" type="file" bind:files on:change={parseFiles} />
  </lable>
  <button class="btn variant-filled-primary" type="submit">
    Tạo chương trình học
  </button>
</form>
