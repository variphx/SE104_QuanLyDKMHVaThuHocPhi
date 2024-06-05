<script>
  import { goto } from "$app/navigation";

  async function submitHandler() {
    await goto("/admin");
  }
</script>

<form
  on:submit|preventDefault={submitHandler}
  class="variant-ghost-surface mx-4 p-12 rounded-2xl"
>
  <label class="label mb-4">
    <span class="mb-2"> Mã công chức/sinh viên </span>
    <input type="text" class="input" />
  </label>

  <label class="label mb-4">
    <span class="mb-2"> Mật khẩu </span>
    <input type="password" class="input" />
  </label>

  <button type="submit" class="btn variant-filled-primary"> Đăng nhập </button>
</form>
