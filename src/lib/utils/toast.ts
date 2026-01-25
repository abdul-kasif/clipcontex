import {toast, type Renderable} from "svelte-french-toast";
import type { ToastStatus } from "$lib/stores/types";

export function showToast(status: ToastStatus, message: Renderable){
    const config = {
      duration: 1500,
      style:
        "background: var(--bg-primary); border: 1px var(--border-colour); font-size: 0.75rem; color: var(--text-primary); font-weight: 500;",
    };

    switch(status) {
        case "success":
            toast.success(message, config);
            break;
        case "error":
            toast.error(message, config);
            break;  
        default:
            toast(message, config);      
    }
}
