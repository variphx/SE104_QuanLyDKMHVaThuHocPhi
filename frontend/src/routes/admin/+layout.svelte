<script>
  import { AppBar } from "@skeletonlabs/skeleton";
</script>

<AppBar>
  <svelte:fragment slot="lead">(uit icon)</svelte:fragment>
  <span>
    <a href="/admin">Trang chủ</a>
    <a href="/admin/sinh-vien">Sinh viên</a>
    <a href="/admin/hoc-phi">Học phí</a>
    <a href="/admin/chuong-trinh-hoc">Chương trình học</a>
    <a href="/admin/hoc-phan">Học phần</a>
  </span>
  <svelte:fragment slot="trail">(user icon)</svelte:fragment>
</AppBar>

<slot></slot>
