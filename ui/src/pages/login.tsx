import React from "react";
import { Link } from "react-router-dom";
import { useAuthContext } from '../context/AuthContext';
import { useForm, SubmitHandler } from "react-hook-form";

type Inputs = {
    email: string,
    password: string,
};

export const Login: React.FC = () => {
    const { login } = useAuthContext();

    const { register, handleSubmit, formState: { errors } } = useForm<Inputs>();
    const onSubmit: SubmitHandler<Inputs> = data => {
        login({
            email: data.email,
            password: data.password,
        })
    };

    return (
        <div style={{ display: "flex", flexDirection: "column" }}>
            <h1>ログイン</h1>
            <form onSubmit={handleSubmit(onSubmit)}>
                <div>
                    <label>Email:</label>
                    <input id="email" {...register("email")} />
                </div>
                <div>
                    <label>Password:</label>
                    <input
                        type="password"
                        id="password"
                        {...register("password")}
                    />
                </div>
                <div>
                    <button type="submit">ログイン</button>
                </div>
            </form>
            <div>
                ユーザ登録は<Link to="/signup">こちら</Link>
            </div>
        </div>
    );
};
