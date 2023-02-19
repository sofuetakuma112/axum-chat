import React from 'react'
import { TextInputWithLabel } from './TextInputWithLabel'
import { useForm, SubmitHandler } from 'react-hook-form'
import { SignUpPayload } from '../../hooks/useAuth'
import { Link } from 'react-router-dom'
import { Button } from './Button'

type Inputs = {
    name: string,
    email: string,
    password: string,
};

type Props = {
    signup: (data: SignUpPayload) => Promise<void>
}

export const SignUpForm: React.FC<Props> = ({ signup }) => {
    const { register, handleSubmit, formState: { errors } } = useForm<Inputs>();
    const onSubmit: SubmitHandler<Inputs> = data => {
        signup({
            name: data.name,
            email: data.email,
            password: data.password,
        });
    };

    return (
        <div className="min-h-screen flex flex-col items-center justify-center bg-gray-300">
            <div className="flex flex-col bg-white shadow-md px-4 sm:px-6 md:px-8 lg:px-10 py-8 rounded-md w-full max-w-md">
                <div className="font-medium self-center text-xl sm:text-2xl uppercase text-gray-800">
                    新規登録
                </div>
                <div className="mt-10">
                    <form onSubmit={handleSubmit(onSubmit)}>
                        <TextInputWithLabel
                            labelText="ユーザー名:"
                            id="name"
                            {...register('name')}
                            svgIcon={
                                <svg className="h-6 w-6"
                                    fill="none"
                                    strokeLinecap="round"
                                    strokeLinejoin="round"
                                    strokeWidth={2}
                                    viewBox="0 0 24 24"
                                    stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"></path>
                                </svg>
                            }
                        />
                        <TextInputWithLabel
                            labelText="メールアドレス:"
                            id="email"
                            {...register('email')}
                            svgIcon={
                                <svg
                                    className="h-6 w-6"
                                    fill="none"
                                    strokeLinecap="round"
                                    strokeLinejoin="round"
                                    strokeWidth={2}
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
                                </svg>
                            }
                        />
                        <TextInputWithLabel
                            labelText="パスワード:"
                            type="password"
                            id="password"
                            {...register('password')}
                            svgIcon={
                                <svg
                                    className="h-6 w-6"
                                    fill="none"
                                    strokeLinecap="round"
                                    strokeLinejoin="round"
                                    strokeWidth={2}
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                </svg>
                            }
                        />
                        <div className="flex w-full">
                            <Button>新規登録</Button>
                        </div>
                    </form>
                </div>
                <div className="flex justify-center items-center mt-6">
                    <Link to="/login" className='inline-flex items-center font-bold text-blue-500 hover:text-blue-700 text-xs text-center'>
                        <span>
                            <svg
                                className="h-6 w-6"
                                fill="none"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth={2}
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                            </svg>
                        </span>
                        <span className="ml-2">アカウントをお持ちの方</span>
                    </Link>
                </div>
            </div>
        </div>
    )
}
