import React from "react";
import { Link } from "react-router-dom";
import { useAuthContext } from '../context/AuthContext';
import { useForm, SubmitHandler } from "react-hook-form";

type Inputs = {
    name: string,
    email: string,
    password: string,
};

export const Signup: React.FC = () => {
    const { signup } = useAuthContext();

    const { register, handleSubmit, formState: { errors } } = useForm<Inputs>();
    const onSubmit: SubmitHandler<Inputs> = data => {
        signup({
            name: data.name,
            email: data.email,
            password: data.password,
        });
    };

    return (
        <div style={{ display: "flex", flexDirection: "column" }}>
            <h1>ユーザ登録</h1>
            <form onSubmit={handleSubmit(onSubmit)}>
                <div>
                    <label htmlFor="name">Name:</label>
                    <input id="name" {...register("name")} />
                </div>
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
                    <button type="submit">ユーザ登録</button>
                </div>
            </form>
            <div>
                ログインは<Link to="/login">こちら</Link>
            </div>
        </div>
    );
};