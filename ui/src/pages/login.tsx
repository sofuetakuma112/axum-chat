import React from "react";
import { useAuthContext } from '../context/AuthContext';
import { LoginForm } from "../components/Auth/LoginForm";

export const Login: React.FC = () => {
    const { login } = useAuthContext();

    return (
        <LoginForm login={login} />
    );
};
