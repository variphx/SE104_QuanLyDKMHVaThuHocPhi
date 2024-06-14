<script lang="ts">
  interface LoginPayload {
    username: String;
    password: String;
  }

  let payload: LoginPayload = {
    username: "",
    password: "",
  };

  async function submitHandler() {
    const request = await fetch("http://localhost:8080/api/user/get", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    if (!request.ok) {
      throw new Error(request.statusText);
    }
  }
</script>

<!-- <form on:submit|preventDefault={submitHandler}> -->
<form method="POST" action="?/login">
  <lable class="label">
    <span> Mã công chức/sinh viên </span>
    <input
      name="username"
      type="text"
      class="input"
      bind:value={payload.username}
    />
  </lable>

  <lable class="label">
    <span> Mật khẩu </span>
    <input
      name="password"
      type="password"
      class="input"
      bind:value={payload.password}
    />
  </lable>

  <lable class="lable">
    <input class="checkbox" type="checkbox" />
    <span> Nhớ tài khoản </span>
  </lable>

  <div class="min-w-full">
    <div class="w-1/4 mx-auto variant-filled-primary rounded-full">
      <button class="btn text-center w-full" type="submit"> Đăng nhập </button>
    </div>
  </div>
</form>
