import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ cookies }) => {
    if (cookies.get("sessionid") && cookies.get("user") != "admin") {
        return {};
    } else {
        redirect(302, '/login')
    }
}