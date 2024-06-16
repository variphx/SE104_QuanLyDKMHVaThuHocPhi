import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
  const session_id_str = cookies.get("session_id");

  if (!session_id_str) {
    redirect(302, "/");
  }

  const session_id = new Number(session_id_str);

  const username_response = await fetch(
    "http://localhost:8080/api/session/get",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(session_id),
    }
  );

  if (!username_response.ok) {
    throw new Error(await username_response.text());
  }

  const username = await username_response.json();

  return {
    username: username,
  };
};
