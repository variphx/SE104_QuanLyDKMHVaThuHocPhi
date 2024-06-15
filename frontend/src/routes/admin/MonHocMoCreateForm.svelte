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
    let success_count = 0;
    let failure_count = 0;

    for (const payload of payloads) {
      const request = await fetch("http://localhost:8080/api/mon-hoc-mo/post", {
        headers: {
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(payload),
      });

      if (request.ok) {
        success_count += 1;
      } else {
        console.log(await request.text());
        failure_count += 1;
      }
    }

    alert(`${success_count} môn học tạo thành công, ${failure_count} thất bại`);
  }
</script>

<form on:submit={submitHandler}>
  <label class="lable mb-4">
    <div class="flex flex-col mb-4">
      <span
        class="h2 font-bold mx-auto rounded-full px-8 py-2 variant-glass-surface"
      >
        Môn học mở
      </span>
    </div>
    <input class="input" type="file" bind:files on:change={parseFiles} />
  </label>
  <div class="flex flex-col">
    <button type="submit" class="mx-auto btn variant-filled-primary">
      Mở môn học
    </button>
  </div>
</form>
