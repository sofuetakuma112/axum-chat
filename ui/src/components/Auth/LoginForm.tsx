import React from 'react'
import { TextInputWithLabel } from '../Forms/TextInputWithLabel'
import { useForm, SubmitHandler } from 'react-hook-form'
import { LoginPayload } from '../../hooks/useAuth'
import { Link } from 'react-router-dom'
import { Button } from '../Forms/Button'

type Inputs = {
    email: string
    password: string
}

type Props = {
    login: (data: LoginPayload) => Promise<void>
}

export const LoginForm: React.FC<Props> = ({ login }) => {
    const {
        register,
        handleSubmit,
        formState: { errors }
    } = useForm<Inputs>()
    const onSubmit: SubmitHandler<Inputs> = (data) => {
        login({
            email: data.email,
            password: data.password
        })
    }

    return (
        <div className="min-h-screen flex flex-col items-center justify-center bg-gray-300">
            <div className="flex flex-col bg-white shadow-md px-4 sm:px-6 md:px-8 lg:px-10 py-8 rounded-md w-full max-w-md">
                <div className="font-medium self-center text-xl sm:text-2xl uppercase text-gray-800">
                    ログイン
                </div>
                <div className="mt-10">
                    <form onSubmit={handleSubmit(onSubmit)}>
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
                        <div className="flex items-center mb-6 -mt-4">
                            <div className="flex ml-auto">
                                <a
                                    href="#"
                                    className="inline-flex text-xs sm:text-sm text-blue-500 hover:text-blue-700"
                                >
                                    パスワードを忘れた方はこちら
                                </a>
                            </div>
                        </div>
                        <div className="flex w-full">
                            <Button>ログイン</Button>
                        </div>
                    </form>
                </div>
                <div className="flex justify-center items-center mt-6">
                    <Link to="/signup" className='inline-flex items-center font-bold text-blue-500 hover:text-blue-700 text-xs text-center'>
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
                        <span className="ml-2">アカウントをお持ちでない方</span>
                    </Link>
                </div>
            </div>
        </div>
    )
}
