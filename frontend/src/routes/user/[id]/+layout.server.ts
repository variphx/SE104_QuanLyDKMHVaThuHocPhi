import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ cookies }) => {
  const user_id = cookies.get("user_id");

  if (!user_id) {
    return;
  }

  return {
    user_id: user_id,
  };
};
