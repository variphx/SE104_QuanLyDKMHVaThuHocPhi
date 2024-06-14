import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ cookies }) => {
    const session_id = cookies.get('session_id');
    if (!session_id) {
        throw redirect(302, '/login');
    }
}