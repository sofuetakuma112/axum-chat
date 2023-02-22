import { Size } from "../types/style";

export const sizeToClassName = (size: Size) => {
    if (size === "sm") {
        return "w-4 h-4"
    } else if (size === "md") {
        return "w-6 h-6"
    } else if (size == "lg") {
        return "w-8 h-8"
    } else {
        return "w-10 h-10"
    }
}