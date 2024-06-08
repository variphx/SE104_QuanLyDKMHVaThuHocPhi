<script lang="ts">
  import { goto } from "$app/navigation";

  interface SessionCreatePayload {
    username: String;
    password: String;
  }

  let payload: SessionCreatePayload = {
    username: "",
    password: "",
  };

  async function submitHandler() {
    const session_create_result = await fetch(
      "http://localhost:8080/api/user/get",
      {
        headers: {
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(payload),
      },
    )
      .then((result) => result.json())
      .catch((err) => console.log(err));

    if (session_create_result["is_success"]) {
      await goto("/admin");
    } else {
      alert("Wrong password");
    }
  }
</script>

<form
  on:submit|preventDefault={submitHandler}
  class="variant-ghost-surface mx-4 p-12 rounded-2xl"
>
  <label class="label mb-4">
    <span class="mb-2"> Mã công chức/sinh viên </span>
    <input type="text" class="input" bind:value={payload.username} />
  </label>

  <label class="label mb-4">
    <span class="mb-2"> Mật khẩu </span>
    <input type="password" class="input" bind:value={payload.password} />
  </label>

  <button type="submit" class="btn variant-filled-primary"> Đăng nhập </button>
</form>
