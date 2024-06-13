<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  onMount(async () => {
    await goto("/admin/chuong-trinh-hoc/lap-ho-so");
  });
</script>
