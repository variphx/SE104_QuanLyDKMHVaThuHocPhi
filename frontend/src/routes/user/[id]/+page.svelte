<script lang="ts">
  import type { PageData } from "../$types";

  export let data: PageData;
</script>

<div class="min-h-[80vh] grid grid-cols-2">
  <div class="my-auto mx-auto">
    <a class="card p-52 h3 variant-soft-surface" href={`./${data.id}/dang-ky`}
      >Đăng ký môn học</a
    >
  </div>
  <div class="my-auto mx-auto">
    <a class="card p-52 h3 variant-soft-surface" href={`./${data.id}/hoc-phi`}
      >Tra cứu học phí</a
    >
  </div>
</div>
