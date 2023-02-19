import React from "react";
import { useAuthContext } from '../context/AuthContext';
import { SignUpForm } from "../components/Auth/SignUpForm";

export const Signup: React.FC = () => {
    const { signup } = useAuthContext();

    return (
        <SignUpForm signup={signup} />
    );
};