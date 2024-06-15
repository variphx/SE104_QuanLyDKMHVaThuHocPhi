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
      },

      header: true,
      worker: true,
      skipEmptyLines: true,
    });
  }

  async function submitHandler() {
    let success_count = 0;
    let failure_count = 0;

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
        success_count += 1;
      } else {
        console.log(await request.text());
        failure_count += 1;
      }
    }

    alert(
      `${success_count} chương trình học tạo thành công, ${failure_count} thất bại`,
    );
  }
</script>

<form on:submit={submitHandler}>
  <label class="lable mb-4">
    <div class="flex flex-col mb-4">
      <span
        class="h2 font-bold mx-auto rounded-full px-8 py-2 variant-glass-surface"
      >
        Chương trình học
      </span>
    </div>
    <input class="input" type="file" bind:files on:change={parseFiles} />
  </label>
  <div class="flex flex-col">
    <button type="submit" class="mx-auto btn variant-filled-primary">
      Tạo chương trình học
    </button>
  </div>
</form>
