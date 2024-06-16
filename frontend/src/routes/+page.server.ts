import { redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

import argon2 from "@node-rs/argon2";

export const load: PageServerLoad = async ({}) => {};

export const actions = {
  login: async ({ request, cookies, fetch }) => {
    const data = await request.formData();

    const username = data.get("username")?.toString();
    const password = data.get("password")?.toString();

    if (!username) {
      return {
        is_username_input: false,
      };
    }

    if (!password) {
      return {
        is_password_input: false,
      };
    }

    const password_hash_response = await fetch(
      "http://localhost:8080/api/user/get",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(username),
      }
    );

    if (!password_hash_response.ok) {
      throw new Error(await password_hash_response.text());
    }

    const password_hash = await password_hash_response.json();

    const is_success = await argon2.verify(password_hash, password);

    if (is_success) {
      const session_id = Date.now();
      const session_id_request = await fetch(
        "http://localhost:8080/api/session/post",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            session_id: session_id,
            username: username,
          }),
        }
      );

      if (!session_id_request.ok) {
        throw new Error(await session_id_request.text());
      }

      if (session_id_request.status === 201) {
        cookies.set("session_id", session_id.toString(), {
          path: "/",
          expires: new Date(Date.now() + 60 * 15 * 1000),
          httpOnly: true,
          secure: true,
        });

        redirect(302, `/user/${username}`);
      }
    }

    return {
      is_login_sucess: is_success,
    };
  },
} satisfies Actions;
