<script lang="ts">
  import type { LayoutData } from "./$types";
  import { AppRail, AppRailAnchor } from "@skeletonlabs/skeleton";
  import { page } from "$app/stores";

  export let data: LayoutData;
  const user_id = data.user_id;
</script>

<div class="w-full h-screen grid grid-cols-[auto_1fr]">
  <AppRail width="w-36">
    <svelte:fragment slot="lead">
      <AppRailAnchor
        href="/user/{user_id}"
        selected={$page.url.pathname === `/user/${user_id}`}
      >
        <svelte:fragment slot="lead">(icon)</svelte:fragment>
        <span>Trang chủ</span>
      </AppRailAnchor>
    </svelte:fragment>
    <AppRailAnchor
      href="/user/{user_id}/hoc-phan/dang-ky"
      selected={$page.url.pathname === `/user/${user_id}/hoc-phan/dang-ky`}
    >
      <svelte:fragment slot="lead">(icon)</svelte:fragment>
      <span>Đăng ký</span>
    </AppRailAnchor>
  </AppRail>

  <slot></slot>
</div>
