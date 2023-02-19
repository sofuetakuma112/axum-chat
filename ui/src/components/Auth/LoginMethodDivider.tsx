import React from 'react'

export const LoginMethodDivider: React.FC = () => {
    return (
        <div className="relative mt-10 h-px bg-gray-300">
            <div className="absolute left-0 top-0 flex justify-center w-full -mt-2">
                <span className="bg-white px-4 text-xs text-gray-500 uppercase">Or Login With Email</span>
            </div>
        </div>
    )
}
