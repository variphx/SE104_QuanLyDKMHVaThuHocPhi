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
  <label class="lable mb-4">
    <div class="flex flex-col mb-4">
      <span
        class="h2 font-bold mx-auto rounded-full px-8 py-2 variant-glass-surface"
      >
        Môn học
      </span>
    </div>
    <input class="input" type="file" bind:files on:change={parseFiles} />
  </label>
  <div class="flex flex-col">
    <button type="submit" class="mx-auto btn variant-filled-primary">
      Tạo môn học
    </button>
  </div>
</form>
