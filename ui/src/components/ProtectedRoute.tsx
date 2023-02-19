import { Navigate } from "react-router-dom";
import { useAuthContext } from "../context/AuthContext";

type Props = {
    children: React.ReactNode
}

export const ProtectedRoute: React.FC<Props> = ({ children }) => {
    const { user } = useAuthContext();

    if (user === undefined) {
        // userステートの初期値はundefined
        return <></>;
    }

    if (!user) {
        // user情報の取得に失敗した場合はステートはnull
        return <Navigate to="/login" />;
    }
    return <>{children}</>;
};
