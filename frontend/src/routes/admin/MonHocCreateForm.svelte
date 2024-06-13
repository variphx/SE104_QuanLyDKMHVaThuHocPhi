<script lang="ts">
  import Papa from "papaparse";

  interface MonHocCreatePayload {
    id: string;
    ten: string;
    loai: string;
    so_tiet: string;
  }

  let files: FileList;
  let payloads: MonHocCreatePayload[];

  async function parseFiles() {
    if (files) {
      const file: File = files[0];
      Papa.parse(file, {
        complete: function (result: Papa.ParseResult<MonHocCreatePayload>) {
          payloads = result.data;
          console.log(payloads);
        },

        header: true,
        worker: true,
        skipEmptyLines: true,
      });
    }
  }

  async function submitHandler() {
    for (let payload of payloads) {
      const request = await fetch("http://localhost:8080/api/mon-hoc/post", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
      });

      if (request.ok) {
        console.log("Tạo môn học thành công");
      } else {
        console.log("Tạo môn học thất bại");
      }
    }
  }
</script>

<form on:submit={submitHandler}>
  <lable class="lable">
    <span> Môn học </span>
    <input class="input" type="file" bind:files on:change={parseFiles} />
  </lable>
  <button class="btn variant-filled-primary" type="submit"> Tạo </button>
</form>
