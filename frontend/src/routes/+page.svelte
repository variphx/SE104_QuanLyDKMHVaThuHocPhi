<script lang="ts">
    interface LoginPayload {
        username: String;
        password: String;
    }

    let login_payload: LoginPayload = {
        username: "22520278",
        password: "password",
    };

    async function handleSubmit() {
        const result = await fetch("http://localhost:8080/api/user/session", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(login_payload),
        }).then((result) => result.json());

        console.log(result["is_success"]);
    }
</script>

<button on:click|preventDefault={handleSubmit}> Click Here to test </button>
