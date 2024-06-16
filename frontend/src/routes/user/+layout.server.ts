import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
  // const session_id = cookies.get("session_id");
  // if (!session_id) {
  //   redirect(307, "/");
  // }
  // const session_fetch = await fetch("http://localhost:8080/api/session/get", {
  //   method: "POST",
  //   headers: {
  //     "Content-Type": "application/json",
  //   },
  //   body: JSON.stringify({
  //     session_id,
  //   }),
  // });
  // if (!session_fetch.ok) {
  //   console.log(await session_fetch.text());
  //   redirect(307, "/");
  // }
  // const session_data = await session_fetch.json();
};
